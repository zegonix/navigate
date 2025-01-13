mod arguments;
mod config;
mod bookmarks;
mod stack;
mod debug;

use arguments::*;
use clap::Parser;
use config::*;
use bookmarks::*;
use config_parser::*;
use stack::Stack;
use std::env::{current_dir, var};
use std::io::{Error, Result};
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn main() -> Result<()> {
    let style_error =
        generate_style_sequence(Some(STYLES.set.bold), Some(COLORS.fg.red), None);
    let args = match Arguments::try_parse() {
        Ok(a) => a,
        Err(e) => {
            print!("echo '{}{}{}' && false", style_error, e, RESET_SEQ);
            return Ok(());
        }
    };
    let config = match Config::new(true) {
        Ok(value) => value,
        Err(error) => {
            print!("echo '{}{}{}' && false", style_error, error, RESET_SEQ);
            return Ok(());
        }
    };
    let mut bookmarks = match Bookmarks::new() {
        Ok(value) => value,
        Err(error) => {
            print!("echo '{}{}{}' && false", style_error, error, RESET_SEQ);
            return Ok(());
        }
    };
    let mut stack = match Stack::new(args.pid) {
        Ok(stack) => stack,
        Err(_) => {
            print!(
                "echo '{}-- failed to build stack{}' && false",
                style_error, RESET_SEQ
            );
            return Err(Error::other(""));
        }
    };
    let res = match args.action {
        Action::push(push_args) => handle_push(&push_args, &config, &mut stack),
        Action::pop(pop_args) => handle_pop(&pop_args, &config, &mut stack),
        Action::stack(stack_args) => handle_stack(&stack_args, &config, &mut stack),
        Action::bookmark(bookmark_args) => handle_bookmark(&bookmark_args, &config, &mut bookmarks, &mut stack),
        // Action::config(config_args) => handle_config(&config_args, &config),
    };

    if res.is_err() {
        print!(
            "echo '{}{}{}' && false",
            style_error,
            res.unwrap_err(),
            RESET_SEQ,
        );
    }
    Ok(())
}

fn handle_push(args: &PushArgs, config: &Config, stack: &mut Stack) -> Result<()> {
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
    push_path(&path, stack, config)?;
    Ok(())
}

fn handle_pop(args: &PopArgs, config: &Config, stack: &mut Stack) -> Result<()> {
    let mut num : Option<usize> = None;
    if let Some(a) = &args.action {
        match a {
            PopAction::all => num = Some(0),
        }
    } else if let Some(n) = &args.num_entries {
        num = Some(*n);
    }
    let path = stack.pop_entry(num)?;
    if config.settings.general.show_stack_on_push {
        print!("echo '{}' && ", stack.to_formatted_string(&config.settings)?);
    }
    println!(
        "cd -- {}",
        match path.to_str() {
            Some(value) => value,
            None => return Err(Error::other("-- failed to print popped path as string")),
        }
    );
    Ok(())
}

fn handle_stack(args: &StackArgs, config: &Config, stack: &mut Stack) -> Result<()> {
    if args.stack_action.is_some() {
        match args.stack_action.clone().unwrap() {
            StackAction::clear(_) => return stack.clear_stack(&config.settings),
        }
    }
    // retrieve stack
    let output: String = stack.to_formatted_string(&config.settings)?;
    print!("echo '{}'", output);
    Ok(())
}

fn handle_bookmark(args: &BookmarkArgs, config: &Config, bookmarks: &mut Bookmarks, stack: &mut Stack) -> Result<()> {
    // if args.bookmark_action.is_some() {
    if let Some(action) = &args.bookmark_action {
        match action {
            BookmarkAction::list(_) => list_bookmarks(config, bookmarks)?,
            BookmarkAction::add(args) => add_bookmarks(args, config, bookmarks)?,
            BookmarkAction::remove(args) => remove_bookmarks(args, config, bookmarks)?,
        };
    } else if args.name.is_some() { // handle `change to bookmark`
        let path = bookmarks.get_path_by_name(args.name.as_ref().unwrap())?;
        push_path(&path, stack, config)?;
    } else {
        return Err(Error::other(
            "-- provide either a `subcommand` or a `bookmark name`",
        ));
    }
    Ok(())
}

// fn handle_config(args: &ConfigArgs, config: &Config) -> Result<()> {
//     match args {
//         ConfigAction::show => println!("echo '{}'", config.to_formatted_string()),
//     }
//     Ok(())
// }

fn list_bookmarks(config: &Config, bookmarks: &mut Bookmarks) -> Result<()> {
    let output = bookmarks.to_formatted_string(&config.settings)?;
    println!("echo '{}'", output);
    Ok(())
}

fn add_bookmarks(args: &BookmarkSubArgs, config: &Config, bookmarks: &mut Bookmarks) -> Result<()> {
    let mut path = match args.path.clone() {
        Some(value) => value,
        None => return Err(Error::other("-- missing path argument")),
    };
    path = match path.canonicalize() {
        Ok(value) => value,
        Err(error) => return Err(Error::other(error.to_string())),
    };
    bookmarks.add_bookmark(&args.name, &path)?;
    Ok(())
}

fn remove_bookmarks(args: &BookmarkSubArgs, config: &Config, bookmarks: &mut Bookmarks) -> Result<()> {
    bookmarks.remove_bookmark(&args.name)?;
    Ok(())
}

/// push path to stack and print command to navigate to provided path
fn push_path(path: &Path, stack: &mut Stack, config: &Config) -> Result<()> {
    if !path.is_dir() {
        return Err(Error::other("-- invalid path argument"));
    }
    let current_path = current_dir()?;
    let next_path = path.canonicalize()?;
    stack.push_entry(&current_path)?;
    if config.settings.general.show_stack_on_push {
        print!("echo '{}' && ", stack.to_formatted_string(&config.settings)?);
    }
    println!(
        "cd -- {}",
        match next_path.to_str() {
            Some(value) => value,
            None => return Err(Error::other("-- failed to print provided path as string")),
        }
    );
    Ok(())
}
