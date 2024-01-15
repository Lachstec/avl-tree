use clap::Parser;
use std::path;

mod avl_tree;

/// Program to visualize AVL-Trees
#[derive(Debug, Parser)]
struct Args {
    /// Print intermediate Trees
    #[arg(short = 'i')]
    intermediates: bool,
    /// Output directory. Defaults to current working directory.
    #[arg(short = 'o')]
    output_directory: Option<path::PathBuf>,
    /// Values to put into the Tree.
    #[arg(short = 'v')]
    values: Vec<i32>,
}

fn main() {
    let args = Args::parse();
    println!("Intermediates: {}", args.intermediates);
}
