// #![allow(warnings)] 
// #![allow(unused)]

mod huffman;

use std::fs;
use std::io::Result;
use std::path::PathBuf;

use clap::Parser;

use crate::huffman::huffman::Huffman;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
        /// Input file path
        #[arg(short, long)]
        input : PathBuf,
    
        /// Output file path
        #[arg(short, long)]
        output : PathBuf,
}


fn main() -> Result<()>{

    let args = Cli::parse();

    println!("Encoding");
    let mut data = fs::read_to_string(&args.input)?;
    let encoding = Huffman::encode(&mut data);
    fs::write(&args.output, &encoding)?;

    println!("Decoding");
    let mut data = fs::read(&args.output)?;
    let result = Huffman::decode(&mut data);
    fs::write("Proof.txt", &result)?;

    Ok(())
}