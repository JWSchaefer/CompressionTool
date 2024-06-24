
use super::table::table::Table;
use super::table::weight::Weight;
use super::table::lookup::Lookup;
use super::tree::tree::BinaryTree;
use super::file::encoder::Encoder;
use super::file::decoder::Decoder;
use super::file::constant::END_CHAR;


pub struct Huffman {
    table : Table,
    tree  : BinaryTree
}

impl Huffman {


    pub fn from_weights(weights : Lookup<Weight>) -> Self{

        let tree = BinaryTree::new(&weights);

        let encodings = tree.get_encodings();

        let decodings = BinaryTree::get_decodings(&encodings);

        let table = Table::new(weights, encodings, decodings);

        Self { table, tree }
    }

    pub fn train(data : &String) -> Self {

        let mut weights = Lookup::<Weight>::new(0);

        for c in data.chars() {
            weights.set(&c, &(weights.lookup(&c) + 1));
        }

        weights.set(&END_CHAR, &1);

        Self::from_weights(weights)
    }

    pub fn encode(data : &mut String) -> Vec<u8> {

        let huffman = Self::train(&data);
        let mut encoder = Encoder::new();

        // File Signature
        encoder.write_signature();

        // Version
        encoder.write_version();

        // Weights
        encoder.write_weights(&huffman.table.weights);
        
        // File data
        encoder.write_string(&huffman.table.encodings, data);

        encoder.data
    }

    pub fn decode(data : &Vec<u8>) -> String {

        let decoder = Decoder::new(data.to_vec());

        // Weights
        let (weights, head) = decoder.read_weights();

        // Build tree
        let huffman = Huffman::from_weights(weights);

        decoder.read_body(&huffman.table.decodings, head)

    }

}


 