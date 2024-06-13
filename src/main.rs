// #![allow(warnings)] 
#![allow(unused)]

mod huffman;

use std::fs;
use std::time::Instant;
use std::path::PathBuf;

use clap::Parser;

use huffman::encoder::encoder::Encoder;


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


fn main() -> std::io::Result<()>{

    let args = Cli::parse();
    let now = Instant::now();

    let its : u32 = 100;
    for _ in 0..its {
        let data    = fs::read_to_string(&args.input)?;
        let encoder = Encoder::new(&data);
        let buffer  = encoder.encode(&data);
        let _file   = fs::write(&args.output, buffer.get_data())?;
    }

    let elapsed = now.elapsed();
    println!("{}\t- {:.2?}", &args.input.file_name().unwrap().to_str().unwrap(), elapsed / its);

    Ok(())
}