mod arguments;
mod config;
mod bookmarks;
mod stack;
mod output;
mod util;

use arguments::*;
use clap::Parser;
use config::*;
use bookmarks::*;
use config_parser::*;
use dirs::home_dir;
use output::Output;
use stack::Stack;
use util::to_rooted;
use std::char;
use std::env::current_dir;
use std::io::{Error, Result};
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn main() -> Result<()> {
    let mut output = Output::new();
    let config = match Config::new() {
        Ok(value) => value,
        Err(error) => {
            // config object is not ready at this point so the style
            // has to be created by hand
            output.push_error(&error.to_string());
            output.print_output(None);
            return Ok(())
        }
    };
    let args = match Arguments::try_parse() {
        Ok(a) => a,
        Err(error) => {
            match error.kind() {
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => {
                    output.push_info(&error.to_string());
                },
                clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
                    output.push_warning(&error.to_string());
                },
                _ => {
                    output.push_error(&error.to_string());
                },
            }
            output.print_output(Some(&config));
            return Ok(());
        }
    };
    let mut bookmarks = match Bookmarks::new() {
        Ok(value) => value,
        Err(error) => {
            output.push_error(&error.to_string());
            output.print_output(Some(&config));
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
        Action::configuration => handle_config(&mut output),
    };

    if res.is_err() {
        output.push_error(&res.unwrap_err().to_string());
    }

    // print output and command
    output.print_output(Some(&config));

    Ok(())
}

fn handle_push(args: &PushArgs, config: &Config, stack: &mut Stack, output: &mut Output) -> Result<()> {
    // paths arguments starting with `=` are interpreted as stack entry number
    const PREFIX: char = '=';

    let mut path_string = match args.path.clone() {
        Some(value) => value.join(" "),
        None => {
            String::new()
        }
    };
    let path: PathBuf = if path_string.is_empty() {
        match home_dir() {
            Some(value) => value,
            None => return Err(Error::other("-- failed to determine home directory")),
        }
    } else if path_string.starts_with(PREFIX) {
        path_string = path_string.trim_start_matches(PREFIX).to_string();
        let number: usize = match path_string.parse() {
            Ok(value) => value,
            Err(_) => return Err(Error::other("-- push : failed to convert path argument to number")),
        };
        stack.get_entry_by_number(number)?.to_path_buf()
    } else {
        match PathBuf::from_str(&path_string) {
            Ok(value) => value,
            Err(_) => return Err(Error::other("-- failed to create PathBuf from argument")),
        }
    };
    if let Some(true) = args.show_stack {
        output.push_info(&stack.to_formatted_string(&config)?);
    } else if config.general.show_stack_on_push {
        output.push_info(&stack.to_formatted_string(&config)?);
    }
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
    if let Some(true) = args.show_stack {
        output.push_info(&stack.to_formatted_string(config)?);
    } else if config.general.show_stack_on_push {
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
            StackAction::clear => {
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
    if let Some(action) = &args.bookmark_action {
        match action {
            BookmarkAction::list => list_bookmarks(config, bookmarks, output)?,
            BookmarkAction::add(args) => add_bookmarks(args, config, bookmarks, output)?,
            BookmarkAction::remove(args) => remove_bookmarks(args, config, bookmarks, output)?,
            BookmarkAction::completions => println!("echo '{}'", bookmarks.get_bookmarknames()),
        };
    } else if args.name.is_some() { // handle `change to bookmark`
        let path = bookmarks.get_path_by_name(args.name.as_ref().unwrap())?;
        push_path(&path, stack, config, output)?;
    } else {
        list_bookmarks(config, bookmarks, output)?;
    }
    Ok(())
}

fn handle_config(output: &mut Output) -> Result<()> {
    let config = Config::new();
    output.push_info(&format!("config = {:#?}", config));
    Ok(())
}

fn list_bookmarks(config: &Config, bookmarks: &mut Bookmarks, output: &mut Output) -> Result<()> {
    output.push_info(&bookmarks.to_formatted_string(config)?);
    Ok(())
}

fn add_bookmarks(args: &BookmarkSubArgs, config: &Config, bookmarks: &mut Bookmarks, output: &mut Output) -> Result<()> {
    let mut path : PathBuf = match PathBuf::from_str(&args.path.join(" ")) {
        Ok(value) => value,
        Err(error) => return Err(Error::other(error.to_string())),
    };
    if args.name == "add" || args.name == "remove" {
        return Err(Error::other("-- `add` & `remove` are subcommands and cant be used as bookmarknames"));
    }
    bookmarks.add_bookmark(&args.name, &path)?;

    if config.general.show_entries_on_bookmark {
        output.push_info(&bookmarks.to_formatted_string(config)?);
    } else {
        _ = to_rooted(&mut path);
        output.push_info(&format!("added bookmark `{}{}{} = {}{}{}`.",
            generate_style_sequence(Some(STYLES.set.bold), None, None), args.name, RESET_SEQ,
            generate_style_sequence(Some(STYLES.set.italic), None, None), path.to_str().unwrap(), RESET_SEQ));
    }

    Ok(())
}

fn remove_bookmarks(args: &BookmarkSubArgs, config: &Config, bookmarks: &mut Bookmarks, output: &mut Output) -> Result<()> {
    bookmarks.remove_bookmark(&args.name)?;

    if config.general.show_entries_on_bookmark {
        output.push_info(&bookmarks.to_formatted_string(config)?);
    } else {
        output.push_info(&format!("remove bookmark `{}{}{}`.", generate_style_sequence(Some(STYLES.set.bold), None, None), args.name, RESET_SEQ));
    }
    Ok(())
}

/// push path to stack and print command to navigate to provided path
fn push_path(path: &Path, stack: &mut Stack, _config: &Config, output: &mut Output) -> Result<()> {
    let mut path = path.to_path_buf();
    let mut current_path: PathBuf = current_dir()?;
    to_rooted(&mut path)?;
    to_rooted(&mut current_path)?;
    if !path.is_dir() {
        return Err(Error::other("-- invalid path argument"));
    } else if !(path == current_path) {
        stack.push_entry(&current_path)?;
        output.push_command(&format!("cd -- '{}'", match path.canonicalize()?.to_str() {
            Some(value) => value,
            None => return Err(Error::other("-- failed to print provided path as string")),
        }));
    }
    Ok(())
}
