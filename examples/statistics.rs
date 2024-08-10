use std::fs;
use std::time::Instant;

// use prettytable::*;
use prettytable::{Cell, Row, Table};
use unbytify::bytify;

use compression::huffman::Huffman;

fn main() {
    let path = "examples/resources/Les Misérables.txt";

    let mut table = Table::new();

    // table.set_titles(row!["Character", "Occurences", "Huffman Code"]);

    let original =
        fs::read_to_string(path.to_string()).expect("Could not read the file");

    let original_size = original.as_bytes().len();

    let iterations = 25;

    let encoding = Huffman::encode(&original);

    let encoding_size = encoding.expect("Failed to encode the file.").len();

    let now = Instant::now();
    for _ in 0..iterations {
        _ = Huffman::encode(&original);
    }
    let elapsed = now.elapsed() / iterations;

    let size_ratio = 100.0 * encoding_size as f64 / original_size as f64;

    let (encoding_size, encoding_bytes_suffix) = bytify(encoding_size as u64);
    let (source_size, source_bytes_suffix) = bytify(original_size as u64);

    table.add_row(Row::new(vec![
        Cell::new("Time Taken"),
        Cell::new(&format!("{elapsed:.2?}")),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Original size"),
        Cell::new(&format!("{source_size:.2?} {source_bytes_suffix}")),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Encoded size"),
        Cell::new(&format!("{encoding_size:.2?} {encoding_bytes_suffix}")),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Ratio"),
        Cell::new(&format!("{size_ratio:.1}%")),
    ]));

    table.printstd();
}
