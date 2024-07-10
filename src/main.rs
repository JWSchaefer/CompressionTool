use std::fs;

use compression::huffman::Huffman;

fn main() {
    let path = "/Users/joe/Downloads/h88NQ-EW.html";
    let _ = Huffman::decode(fs::read(path.to_string()).expect("Shite"));
}
