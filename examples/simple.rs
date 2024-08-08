use std::fs;

use compression::huffman::Huffman;

fn main() {
    let path = "/Users/joe/prepos/compression/examples/resources/Notes and Queries, Number 82, May 24, 1851.txt";

    let original =
        fs::read_to_string(path.to_string()).expect("Could not read the file");

    let encoding = Huffman::encode(original.clone()).unwrap();

    let decoded = Huffman::decode(encoding).expect("Failed to decode");

    println!("Sucess? {}", original == decoded);
}
