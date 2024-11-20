mod args;

use std::env;
use clap::Parser;
use args::CommandArgs;


fn main()
{
    let args = CommandArgs::parse();
}
