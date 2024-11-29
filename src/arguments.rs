use clap::{Args, Parser, Subcommand};
use std::{path::PathBuf};


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
#[allow(non_camel_case_types)]
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
    pub path: PathBuf,
}

#[derive(Debug, Clone, Args)]
pub struct PopArgs {
    /// show stack
    #[arg(short, long)]
    pub show_stack: Option<bool>,

    /// pop multiple entries and navigate to last retrieved path
    pub num_entries: Option<usize>,
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
#[allow(non_camel_case_types)]
pub enum StackAction {
    /// clear stack
    clear(ClearArgs),
}

#[derive(Debug, Clone, Args)]
pub struct ClearArgs {}

#[derive(Debug, Clone, Args)]
pub struct BookmarkArgs {
    /// show stack
    #[arg(short, long)]
    pub show_stack: Option<bool>,

    pub mark: String,
}
