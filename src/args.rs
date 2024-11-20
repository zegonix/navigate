use clap:: {Parser, Args, Subcommand};

/// implements stack for cd wrapper script
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CommandArgs
{

    /// subcommand
    #[clap(subcommand)]
    action: Action,

    /// process id of parent shell
    #[arg(long="pid")]
    pid: u32,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Action
{
    /// navigate to path and add current path to the stack
    push(PushArgs),

    /// navigate to last entry in stack and remove it
    pop(PopArgs),

    /// display stack
    stack(StackArgs),

    // bookmark(BookmarkArgs),
}

#[derive(Debug, Clone, Args)]
pub struct PushArgs
{
    /// show stack
    show_stack: Option<bool>,

    /// change to <path>
    path: String,
}

#[derive(Debug, Clone, Args)]
pub struct PopArgs
{
    /// show stack
    #[arg(short, long)]
    show_stack: Option<bool>,
}

#[derive(Debug, Clone, Args)]
pub struct StackArgs
{
    /// hide entry numbers
    #[arg(short, long)]
    hide_numbers: Option<bool>,

    /// show n entries
    #[arg(short, long="lines")]
    lines: Option<u32>,
}