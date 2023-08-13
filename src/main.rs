mod args;

use args::Args;
use clap::Parser;

fn main() {
    let _args = Args::parse();

    println!("Hello, world!");
}
