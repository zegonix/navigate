mod arguments;
mod config;
mod stack;

use arguments::*;
use clap::{Parser};
use stack::Stack;
use std::env::current_dir;
use std::io::{Error, Result};

fn main() -> Result<()> {
    let args = Arguments::parse();
    let mut stack = match Stack::new(args.pid) {
        Ok(stack) => stack,
        Err(_) => return Err(Error::other("-- failed to build stack")),
    };

    match args.action {
        Action::push(push_args) => handle_push(&push_args, &mut stack),
        Action::pop(pop_args) => handle_pop(&pop_args, &mut stack),
        Action::stack(stack_args) => handle_stack(&stack_args, &mut stack),
        Action::bookmark(bookmark_args) => handle_bookmark(&bookmark_args, &mut stack),
    }
}

pub fn handle_push(args: &PushArgs, stack: &mut Stack) -> Result<()> {
    // TODO: handle arguments
    if !args.path.is_dir() {
        return Err(Error::other("-- invalid path argument"));
    }
    let current_path = current_dir()?;
    stack.push_entry(&current_path)?;
    println!("{}", args.path.to_str().unwrap());
    Ok(())
}

pub fn handle_pop(args: &PopArgs, stack: &mut Stack) -> Result<()> {
    // TODO: handle arguments
    let path = stack.pop_entry()?;
    println!("{}", path.to_str().unwrap());
    Ok(())
}

pub fn handle_stack(args: &StackArgs, stack: &mut Stack) -> Result<()> {
    // TODO: handle arguments
    if args.stack_action.is_some() {
        match args.stack_action.clone().unwrap() {
            StackAction::clear(_) => return stack.clear_stack(),
        }
    }
    // retrieve stack
    let output = stack.get_stack()?;
    if output.is_empty() {
        return Err(Error::other("-- the stack is empty"));
    }
    // print stack to standard output
    for (n, item) in output.iter().rev().enumerate() {
        println!("{} {}", n, item.to_str().unwrap());
    }
    Ok(())
}

pub fn handle_bookmark(args: &BookmarkArgs, stack: &mut Stack) -> Result<()> {
    // TODO: handle arguments
    Ok(())
}
