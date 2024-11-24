mod arguments;
mod stack;

use clap::Parser;
use arguments::Arguments;
use stack::Stack;


fn main() {
    let args = Arguments::parse();

    let stack = Stack::new(args.pid);
}