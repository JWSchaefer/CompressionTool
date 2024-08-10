use prettytable::*;
use prettytable::{Cell, Row, Table};
use std::fs;

use compression::huffman::Huffman;

fn char_repr(c: char) -> String {
    match c {
        ' ' => "\\s".to_string(),   // Represent space
        '\n' => "\\n".to_string(),  // Represent newline
        '\t' => "\\t".to_string(),  // Represent tab
        '\\' => "\\\\".to_string(), // Represent backslash
        '\'' => "\\'".to_string(),  // Represent single quote
        '"' => "\\\"".to_string(),  // Represent double quote
        _ if c.is_ascii_graphic() => c.to_string(), // Regular printable characters
        _ => format!("\\u{{{:04X}}}", c as u32),    // Unicode characters
    }
}

fn main() {
    let mut table = Table::new();

    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(row!["Character", "Occurences", "Huffman Code"]);

    let path = "examples/resources/Les Misérables.txt";

    let original =
        fs::read_to_string(path.to_string()).expect("Could not read the file");

    let encoder = Huffman::train(original.clone());

    let encoder_table = encoder.get_table().expect("Failed to extract table");

    for (c, weight, code) in encoder_table {
        table.add_row(Row::new(vec![
            Cell::new(&format!("{}", char_repr(c))),
            Cell::new(&format!("{}", weight)),
            Cell::new(&format!("{:>31}", code.get_string())),
        ]));
    }

    table.printstd();
}
