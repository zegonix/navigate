mod arguments;
mod config;
mod bookmarks;
mod stack;
mod output;
mod util;
mod debug;

use arguments::*;
use clap::Parser;
use config::*;
use bookmarks::*;
use config_parser::*;
use output::Output;
use stack::Stack;
use std::env::{current_dir, var};
use std::io::{Error, Result};
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn main() -> Result<()> {
    let mut output = Output::new();
    let config = match Config::new(true) {
        Ok(value) => value,
        Err(error) => {
            // config object is not ready at this point so the style
            // has to be created by hand
            print!("echo '{}{}{}' && false", generate_style_sequence(None, Some(COLORS.fg.red), None), error, RESET_SEQ);
            return Ok(())
        }
    };
    let args = match Arguments::try_parse() {
        Ok(a) => a,
        Err(error) => {
            output.push_error(&error.to_string());
            output.print_output(&config);
            return Ok(());
        }
    };
    let mut bookmarks = match Bookmarks::new() {
        Ok(value) => value,
        Err(error) => {
            output.push_error(&error.to_string());
            output.print_output(&config);
            return Ok(());
        }
    };
    let mut stack = match Stack::new(args.pid) {
        Ok(stack) => stack,
        Err(_) => {
            output.push_error(&"-- failed to build stack".to_string());
            return Err(Error::other(""));
        }
    };
    let res = match args.action {
        Action::push(push_args) => handle_push(&push_args, &config, &mut stack, &mut output),
        Action::pop(pop_args) => handle_pop(&pop_args, &config, &mut stack, &mut output),
        Action::stack(stack_args) => handle_stack(&stack_args, &config, &mut stack, &mut output),
        Action::bookmark(bookmark_args) => handle_bookmark(&bookmark_args, &config, &mut bookmarks, &mut stack, &mut output),
    };

    if res.is_err() {
        output.push_error(&res.unwrap_err().to_string());
        output.push_command(&"false".to_owned());
    }

    // print output and command
    output.print_output(&config);

    Ok(())
}

fn handle_push(args: &PushArgs, config: &Config, stack: &mut Stack, output: &mut Output) -> Result<()> {
    let path = match args.path.clone() {
        Some(value) => value,
        None => {
            let home_dir = match var("HOME") {
                Ok(value) => value,
                Err(error) => return Err(Error::other(error.to_string())),
            };
            match PathBuf::from_str(&home_dir) {
                Ok(value) => value,
                Err(error) => return Err(Error::other(error.to_string())),
            }
        }
    };
    push_path(&path, stack, config, output)?;
    Ok(())
}

fn handle_pop(args: &PopArgs, config: &Config, stack: &mut Stack, output: &mut Output) -> Result<()> {
    let mut num : Option<usize> = None;
    if let Some(a) = &args.action {
        match a {
            PopAction::all => num = Some(0),
        }
    } else if let Some(n) = &args.num_entries {
        num = Some(*n);
    }
    let path = stack.pop_entry(num)?;
    if config.general.show_stack_on_push {
        output.push_info(&stack.to_formatted_string(config)?);
    }
    output.push_command(&format!("cd -- {}", match path.to_str() {
        Some(value) => value,
        None => return Err(Error::other("-- failed to print popped path as string")),
    }));
    Ok(())
}

fn handle_stack(args: &StackArgs, config: &Config, stack: &mut Stack, output: &mut Output) -> Result<()> {
    if args.stack_action.is_some() {
        match args.stack_action.clone().unwrap() {
            StackAction::clear(_) => {
                stack.clear_stack()?;
                output.push_info(&"stack cleared.".to_owned());
                return Ok(());
            }
        }
    }
    // retrieve stack
    output.push_info(&stack.to_formatted_string(config)?);
    Ok(())
}

fn handle_bookmark(args: &BookmarkArgs, config: &Config, bookmarks: &mut Bookmarks, stack: &mut Stack, output: &mut Output) -> Result<()> {
    // if args.bookmark_action.is_some() {
    if let Some(action) = &args.bookmark_action {
        match action {
            BookmarkAction::list(_) => list_bookmarks(config, bookmarks, output)?,
            BookmarkAction::add(args) => add_bookmarks(args, config, bookmarks, output)?,
            BookmarkAction::remove(args) => remove_bookmarks(args, config, bookmarks, output)?,
        };
    } else if args.name.is_some() { // handle `change to bookmark`
        let path = bookmarks.get_path_by_name(args.name.as_ref().unwrap())?;
        push_path(&path, stack, config, output)?;
    } else {
        list_bookmarks(config, bookmarks, output)?;
    }
    Ok(())
}

// fn handle_config(args: &ConfigArgs, config: &Config) -> Result<()> {
//     match args {
//         ConfigAction::show => println!("echo '{}'", config.to_formatted_string()),
//     }
//     Ok(())
// }

fn list_bookmarks(config: &Config, bookmarks: &mut Bookmarks, output: &mut Output) -> Result<()> {
    output.push_info(&bookmarks.to_formatted_string(config)?);
    Ok(())
}

fn add_bookmarks(args: &BookmarkSubArgs, config: &Config, bookmarks: &mut Bookmarks, output: &mut Output) -> Result<()> {
    let mut path = match args.path.clone() {
        Some(value) => value,
        None => return Err(Error::other("-- missing path argument")),
    };
    path = match path.canonicalize() {
        Ok(value) => value,
        Err(error) => return Err(Error::other(error.to_string())),
    };
    bookmarks.add_bookmark(&args.name, &path)?;

    if config.general.show_books_on_bookmark {
        output.push_info(&bookmarks.to_formatted_string(config)?);
    } else {
        output.push_info(&format!("added bookmark `{}{}{}`.", generate_style_sequence(Some(STYLES.set.bold), None, None), args.name, RESET_SEQ));
    }

    Ok(())
}

fn remove_bookmarks(args: &BookmarkSubArgs, config: &Config, bookmarks: &mut Bookmarks, output: &mut Output) -> Result<()> {
    bookmarks.remove_bookmark(&args.name)?;

    if config.general.show_books_on_bookmark {
        output.push_info(&bookmarks.to_formatted_string(config)?);
    } else {
        output.push_info(&format!("remove bookmark `{}{}{}`.", generate_style_sequence(Some(STYLES.set.bold), None, None), args.name, RESET_SEQ));
    }
    Ok(())
}

/// push path to stack and print command to navigate to provided path
fn push_path(path: &Path, stack: &mut Stack, config: &Config, output: &mut Output) -> Result<()> {
    if !path.is_dir() {
        return Err(Error::other("-- invalid path argument"));
    }
    let current_path = current_dir()?;
    stack.push_entry(&current_path)?;
    if config.general.show_stack_on_push {
        output.push_info(&stack.to_formatted_string(&config)?);
    }
    output.push_command(&format!("cd -- {}", match path.canonicalize()?.to_str() {
        Some(value) => value,
        None => return Err(Error::other("-- failed to print provided path as string")),
    }));
    Ok(())
}
