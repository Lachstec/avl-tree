use avl_tree::AvlTree;
use clap::{Parser, ValueEnum};
use std::env;
use std::{path, fs::File};
use std::io::Write;
use std::process::ExitCode;
use graphviz_rust::{
    cmd::Format,
    exec_dot
};

mod avl_tree;

/// Program to visualize AVL-Trees
#[derive(Debug, Parser)]
struct Args {
    /// Print intermediate Trees. This generates a file for every value in the tree.
    #[arg(short = 'i')]
    intermediates: bool,
    /// Output directory. Defaults to current working directory.
    #[arg(short = 'o')]
    output_directory: Option<path::PathBuf>,
    /// Values to put into the Tree.
    #[arg(short = 'v', num_args(0..))]
    values: Vec<i32>,
    /// Whether to Output the Tree as SVGs or dotfiles
    #[arg(short = 't')]
    filetype: OutputType,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum OutputType {
    Svg,
    Dotfile,
}

fn generate_files(filetype: OutputType, dotfiles: Vec<String>, path: path::PathBuf) -> std::io::Result<()> {
    match filetype {
        OutputType::Dotfile => {
            for (index, dotfile) in dotfiles.into_iter().enumerate() {
                let mut p = path::PathBuf::from(path.clone());
                p.extend(&[format!("out-{}", index)]);
                let mut file = File::create(p)?;
                file.write_all(&dotfile.as_bytes())?;
            }
        },
        OutputType::Svg => {
            let format = Format::Svg;
            for (index, dotfile) in dotfiles.into_iter().enumerate() {
                let svg = exec_dot(dotfile, vec![format.into()]);
                match svg {
                    Err(e) => return Err(e),
                    Ok(s) => {
                        let mut p = path::PathBuf::from(path.clone());
                        p.extend(&[format!("out-{}.svg", index)]);
                        let mut file = File::create(p)?;
                        file.write_all(&s)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    let args = Args::parse();
    let mut dotfiles: Vec<String> = Vec::new();
    if args.intermediates {
        let mut t = AvlTree::new();
        for (index, value) in args.values.into_iter().enumerate() {
            t.insert(value);
            dotfiles.insert(index, t.as_dotfile().unwrap_or(String::from("")));
        }
        
    } else {
        let t: AvlTree<i32> = args.values
            .into_iter()
            .collect();
        dotfiles.insert(0, t.as_dotfile().unwrap_or(String::from("")));
    };

    match generate_files(args.filetype, dotfiles, args.output_directory.unwrap_or(env::current_dir().unwrap())) {
        Ok(()) => {},
        Err(_) => return ExitCode::FAILURE,
    }

    ExitCode::SUCCESS
}
