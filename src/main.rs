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

pub fn check_encode_speed(in_path : &PathBuf, out_path : &PathBuf) -> Result<()> {

    let its : u32 = 100;
    let now = Instant::now();

    for _ in 0..its {

        let data    = fs::read_to_string(in_path)?;
        let huf     = Huffman::from_string(&data);
        let buffer  = huf.encode(&data);

        fs::write(&out_path, buffer)?;
    }

    let elapsed = now.elapsed();
    println!(
        "{}\t- {:.2?}", 
        &in_path.file_name().unwrap().to_str().unwrap(), 
        elapsed / its
    );

    Ok(())
}

fn encode(in_path : &PathBuf, out_path : &PathBuf) -> Result<()> {

    let data    = fs::read_to_string(in_path)?;
    let huf     = Huffman::from_string(&data);
    let buffer  = huf.encode(&data);

    fs::write(&out_path, buffer.get_data())?;
    
    Ok(())
}

fn decode(in_path : &PathBuf) -> Result<()>{
    let data = fs::read(in_path)?;

    let decoder = Huffman::from_raw(&data);

    Ok(())
}


fn main() -> Result<()>{

    let args = Cli::parse();
    encode(&args.input, &args.output)?;
    decode(&args.output)?;
    // check_encode_speed(&args.input, &args.output)?;
    
    Ok(())
}