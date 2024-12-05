mod arguments;
mod config;
mod format;
mod stack;

use arguments::*;
use clap::Parser;
use config::*;
use format::*;
use stack::Stack;
use std::env::{current_dir, var};
use std::io::{Error, Result};
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn main() -> Result<()> {
    let style_error =
        generate_style_sequence(Some(vec![STYLES.set.bold]), Some(COLORS.fg.red), None);
    let args = match Arguments::try_parse() {
        Ok(a) => a,
        Err(e) => {
            print!("echo '{}{}{}' && false", style_error, e, RESET_SEQ);
            return Ok(());
        }
    };
    let mut config = match Config::new() {
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
        Action::push(push_args) => handle_push(&push_args, &mut config, &mut stack),
        Action::pop(pop_args) => handle_pop(&pop_args, &mut config, &mut stack),
        Action::stack(stack_args) => handle_stack(&stack_args, &mut config, &mut stack),
        Action::bookmark(bookmark_args) => handle_bookmark(&bookmark_args, &mut config, &mut stack),
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

fn handle_push(args: &PushArgs, _config: &mut Config, stack: &mut Stack) -> Result<()> {
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
    push_path(&path, stack)?;
    Ok(())
}

fn handle_pop(args: &PopArgs, _config: &mut Config, stack: &mut Stack) -> Result<()> {
    let path = stack.pop_entry(args.num_entries)?;
    println!(
        "cd -- {}",
        match path.to_str() {
            Some(value) => value,
            None => return Err(Error::other("-- failed to print popped path as string")),
        }
    );
    Ok(())
}

fn handle_stack(args: &StackArgs, config: &mut Config, stack: &mut Stack) -> Result<()> {
    if args.stack_action.is_some() {
        match args.stack_action.clone().unwrap() {
            StackAction::clear(_) => return stack.clear_stack(config),
        }
    }
    // retrieve stack
    let output: String = stack.to_string(None)?;
    print!("echo '{}'", output);
    Ok(())
}

fn handle_bookmark(args: &BookmarkArgs, config: &mut Config, stack: &mut Stack) -> Result<()> {
    // if args.bookmark_action.is_some() {
    if args.bookmark_action.is_some() {
        match args.bookmark_action.clone().unwrap() {
            BookmarkAction::list(_) => list_bookmarks(&mut config)?,
            BookmarkAction::add(args) => add_bookmarks(&args, &mut config)?,
            BookmarkAction::remove(args) => remove_bookmarks(&args, &mut config)?,
        };
    } else if args.name.is_some() {
        let path = match config.get_bookmarks().get(args.name.as_ref().unwrap()) {
            Some(value) => value,
            None => return Err(Error::other("-- requested bookmark does not exist")),
        };
        push_path(path, stack)?;
    } else {
        return Err(Error::other(
            "-- provide either a `subcommand` or a `bookmark name`",
        ));
    }
    Ok(())
}

fn list_bookmarks(config: &mut Config) -> Result<()> {
    let mut buffer = String::new();
    for (mark, path) in config.get_bookmarks() {
        buffer.push_str(&format!("{} : {}\n", mark, path.to_str().unwrap()));
    }
    println!("echo '{}'", buffer);
    Ok(())
}

fn add_bookmarks(args: &BookmarkSubArgs, config: &mut Config) -> Result<()> {
    if args.path.is_none() {
        return Err(Error::other("-- missing path argument"));
    } else {
        config.add_bookmark(&args.name, &args.path.clone().unwrap())?;
    }
    Ok(())
}

fn remove_bookmarks(args: &BookmarkSubArgs, config: &mut Config) -> Result<()> {
    config.remove_bookmark(&args.name)?;
    Ok(())
}

/// push path to stack and print command to navigate to provided path
fn push_path(path: &Path, stack: &mut Stack) -> Result<()> {
    if !path.is_dir() {
        return Err(Error::other("-- invalid path argument"));
    }
    let current_path = current_dir()?;
    let next_path = path.canonicalize()?;
    stack.push_entry(&current_path)?;
    println!(
        "cd -- {}",
        match next_path.to_str() {
            Some(value) => value,
            None => return Err(Error::other("-- failed to print provided path as string")),
        }
    );
    Ok(())
}
