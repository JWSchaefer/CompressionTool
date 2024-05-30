mod huffman;

use std::fs;
use clap::Parser;
use std::time::Instant;

use huffman::tree::Tree;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {

    let args = Cli::parse();

    let data = fs::read_to_string(args.path).unwrap_or(String::from("None"));
    let tree : Tree = Tree::new(&data);
    
    
    println!("Elapsed:\t\t{:.2?}", elapsed);
    println!("Elapsed per iter:\t{:.2?}", elapsed / 1_000);
    
}