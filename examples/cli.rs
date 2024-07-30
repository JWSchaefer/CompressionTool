use std::fs;

use compression::huffman::Huffman;

fn main() {
    let path = "/Users/joe/Downloads/les_mis.huff";
    let result = Huffman::decode(fs::read(path.to_string()).expect("Shite"));
    println!("{result:?}");
}
