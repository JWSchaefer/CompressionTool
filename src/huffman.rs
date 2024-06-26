pub mod bitstream{
    pub mod decode_stream;
    pub mod encode_stream;
}
pub mod file{
    pub mod constant;
    pub mod decoder;
    pub mod encoder;
}

pub mod table{
    pub mod encoding;
    pub mod lookup;
    pub mod table;
    pub mod weight;
}
pub mod tree{
    pub mod internal;
    pub mod leaf;
    pub mod node;
    pub mod tree;
}

use crate::table::table::Table;
use crate::table::lookup::Lookup;
use crate::tree::tree::BinaryTree;
use crate::file::constant::END_CHAR;
use crate::table::weight::Weight;
use crate::file::encoder::Encoder;
use crate::file::decoder::Decoder;

use std::fs;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct Huffman {
    table : Table,
    tree  : BinaryTree
}

#[wasm_bindgen]
impl Huffman {

    fn from_weights(weights : Lookup<Weight>) -> Self{

        let tree = BinaryTree::new(&weights);
        let encodings = tree.get_encodings();
        let decodings = BinaryTree::get_decodings(&encodings);
        let table = Table::new(weights, encodings, decodings);

        Self { table, tree }
    }

    fn gen_weights(data : &String) -> Lookup<Weight> {

        let mut weights = Lookup::<Weight>::new(0);
    
        for c in data.chars() {
            weights.set(&c, &(weights.lookup(&c) + 1));
        }
    
        weights.set(&END_CHAR, &1);
    
        weights
    }

    #[wasm_bindgen(constructor)]
    pub fn train(in_path : String) -> Self {

        let data = fs::read_to_string(&in_path).unwrap();

        let weights = Self::gen_weights(&data);

        Self::from_weights(weights)

    }

    #[wasm_bindgen]
    pub fn encode(in_path : String, out_path : String) {

        let mut data = fs::read_to_string(&in_path).unwrap();

        let weights = Self::gen_weights(&data);
        let huffman = Self::from_weights(weights);

        let mut encoder = Encoder::new();

        // File Signature
        encoder.write_signature();

        // Version
        encoder.write_version();

        // Weights
        encoder.write_weights(&huffman.table.weights);
        
        // File data
        encoder.write_string(&huffman.table.encodings, &mut data);

        fs::write(out_path, encoder.data).unwrap();
    }

    #[wasm_bindgen]
    pub fn decode(in_path : String, out_path : String) {

        let data = fs::read(&in_path).unwrap();

        let decoder = Decoder::new(data.to_vec());

        // Weights
        let (weights, head) = decoder.read_weights();

        // Build tree
        let huffman = Huffman::from_weights(weights);

        let result = decoder.read_body(&huffman.table.decodings, head);

        fs::write(out_path, result).unwrap();

    }

    #[wasm_bindgen]
    pub fn get_tree(&self) -> String {
        self.tree.serialise()
    }

}


 