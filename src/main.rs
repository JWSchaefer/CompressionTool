// #![allow(warnings)] 
#![allow(unused)]

mod huffman;

use std::fs;
use std::io::Result;
use std::time::Instant;
use std::path::PathBuf;

use clap::Parser;

use crate::huffman::huffman::huffman::Huffman;

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

fn encode(in_path : &PathBuf, out_path : &PathBuf) -> Result<()> {

    let mut data = fs::read_to_string(in_path)?;
    let buffer   = Huffman::encode(&mut data);

    fs::write(&out_path, buffer)?;

    Ok(())
}

fn decode(in_path : &PathBuf) -> Result<()>{
    let data = fs::read(in_path)?;
    let result = match Huffman::decode(&data) {
        Ok(result) =>  {},
        Err(message) => panic!("Error Decoding {message}")
    };

    Ok(())
}


fn main() -> Result<()>{

    let args = Cli::parse();
    println!("Encoding");
    encode(&args.input, &args.output)?;
    println!("Decoding");
    decode(&args.output)?;
    
    Ok(())
}