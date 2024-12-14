#![allow(dead_code)]
#![allow(non_camel_case_types)]

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// implements stack for cd wrapper script
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
pub struct Arguments {
    /// process id of parent shell
    #[arg(short, long)]
    pub pid: u32,

    /// subcommand
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Action {
    /// navigate to path and add current path to the stack
    push(PushArgs),

    /// navigate to last entry in stack and remove it
    pop(PopArgs),

    /// show stack
    stack(StackArgs),

    /// navigate to bookmark and add current path to the stack
    bookmark(BookmarkArgs),
}

#[derive(Debug, Clone, Args)]
pub struct PushArgs {
    /// show stack
    #[arg(short, long)]
    pub show_stack: Option<bool>,

    /// change to <path>
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone, Args)]
pub struct PopArgs {
    /// pop multiple values, either a specified amount with a number, or all entries with 'all'
    #[command(subcommand)]
    pub action: Option<PopAction>,

    /// show stack
    #[arg(short, long)]
    pub show_stack: Option<bool>,

    /// pop specified number of entries
    pub num_entries: Option<usize>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum PopAction {
    /// pop all entries and move to first entry in stack
    all,
}

#[derive(Debug, Clone, Args)]
pub struct StackArgs {
    /// hide entry numbers
    #[arg(short = 'n', long)]
    pub hide_numbers: Option<bool>,

    /// show n entries
    #[arg(short, long = "lines")]
    pub lines: Option<u32>,

    /// stack subcommand
    #[command(subcommand)]
    pub stack_action: Option<StackAction>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum StackAction {
    /// clear stack
    clear(EmptyArgs),
}

#[derive(Debug, Clone, Args)]
pub struct BookmarkArgs {
    /// bookmark subcommand
    #[command(subcommand)]
    pub bookmark_action: Option<BookmarkAction>,

    /// name of bookmark to push
    pub name: Option<String>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum BookmarkAction {
    /// list all bookmarks
    list(EmptyArgs),

    /// add a bookmark with `book add <name> <path>`
    add(BookmarkSubArgs),

    /// remove a bookmark by name `book remove <name>`
    remove(BookmarkSubArgs),
}

#[derive(Debug, Clone, Args)]
pub struct BookmarkSubArgs {
    /// name of bookmark to add/remove
    pub name: String,

    /// path of bookmark to add
    pub path: Option<PathBuf>,
}

/// empty struct for subcommands with no arguments
#[derive(Debug, Clone, Args)]
pub struct EmptyArgs {}
