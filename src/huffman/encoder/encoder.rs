use super::buffer::Buffer;
use super::constant::SIGNATURE;

use super::super::table::table::Table;
use super::super::table::lookup::Lookup;
use super::super::table::weight::Weight;
use super::super::tree::tree::BinaryTree;
use super::super::table::encoding::HuffEncoding;

// use byteorder::{ByteOrder, WriteBytesExt, LE, BE};

pub struct Encoder{
    tree  : BinaryTree,
    table : Table
}

impl Encoder {

    pub fn new(data : &String) -> Self {

        let mut weights = Lookup::<Weight>::new(0);

        for c in data.chars() {
            weights.set(&c, &(weights.lookup(&c) + 1));
        }

        let tree = BinaryTree::new(&weights);

        let encodings = tree.get_encodings();

        let table = Table::new(weights, encodings);

        Self { tree, table }
    }

    pub fn encode(&self, data : &String) -> Buffer {

        let mut buffer = Buffer::new(Vec::from(SIGNATURE));

        for c in data.chars() {
            let mut encoding = self.table.encodings.lookup(&c);
            buffer.write(&mut encoding.get_raw());
        }

        let data = buffer.get_data();

        buffer
    }
}