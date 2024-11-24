use clap::{Parser, Args, Subcommand};


/// implements stack for cd wrapper script
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
pub struct Arguments {
    /// subcommand
    #[command(subcommand)]
    pub action: Action,

    /// process id of parent shell
    #[arg(short, long)]
    pub pid: u32,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Action {
    /// navigate to path and add current path to the stack
    push(PushArgs),

    /// navigate to last entry in stack and remove it
    pop(PopArgs),

    /// display stack
    stack(StackArgs),

    /// navigate to bookmark and add current path to the stack
    bookmark(BookmarkArgs),
}

#[derive(Debug, Clone, Args)]
pub struct PushArgs {
    /// show stack
    #[arg(short, long)]
    show_stack: Option<bool>,

    /// change to <path>
    path: String,
}

#[derive(Debug, Clone, Args)]
pub struct PopArgs {
    /// show stack
    #[arg(short, long)]
    show_stack: Option<bool>,
}

#[derive(Debug, Clone, Args)]
pub struct StackArgs {
    /// hide entry numbers
    #[arg(short='n', long)]
    hide_numbers: Option<bool>,

    /// show n entries
    #[arg(short, long="lines")]
    lines: Option<u32>,
}

#[derive(Debug, Clone, Args)]
pub struct BookmarkArgs {
    /// show stack
    #[arg(short, long)]
    show_stack: Option<bool>,
}