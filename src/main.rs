mod arguments;
mod stack;

use std::io::{Result, Error, ErrorKind};
use std::path::PathBuf;
use clap::{FromArgMatches, Parser};
use arguments::*;
use stack::Stack;


fn main() -> Result<()> {
    let args = Arguments::parse();
    let mut stack = Stack::new(args.pid).expect("failed to build stack");

    return match args.action {
        Action::push(push_args) => handle_push(&push_args, &mut stack),
        Action::pop(pop_args) => handle_pop(&pop_args, &mut stack),
        Action::show(show_args) => handle_show(&show_args, &mut stack),
        Action::bookmark(bookmark_args) => handle_bookmark(&bookmark_args, &mut stack),
        _ => Err(Error::new(ErrorKind::Other, "unknown subcommand")),
    };
}

pub fn handle_push (args: &PushArgs, stack: &mut Stack) -> Result<()> { // TODO: handle arguments
    stack.push_entry(&args.path)?;
    println!("{}", args.path.to_str().unwrap());
    return Ok(());
}

pub fn handle_pop (args: &PopArgs, stack: &mut Stack) -> Result<()> { // TODO: handle arguments
    let path = stack.pop_entry()?;
    println!("{}", path.to_str().unwrap());
    return Ok(());
}

pub fn handle_show (args: &ShowArgs, stack: &mut Stack) -> Result<()> { // TODO: handle arguments
    let output = stack.get_stack()?;
    for item in output {
        println!("{}", item.to_str().unwrap());
    }
    return Ok(());
}

pub fn handle_bookmark (args: &BookmarkArgs, stack: &mut Stack) -> Result<()> { // TODO: handle arguments
    return Ok(());
}
