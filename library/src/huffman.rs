use crate::core::code::code::Code;
use crate::core::constant::END_CHAR;
use crate::core::decoder::Decoder;
use crate::core::encoder::Encoder;
use crate::core::lookup::Lookup;
use crate::core::table::Table;
use crate::core::tree::tree::BinaryTree;
use crate::core::weight::Weight;

pub struct Huffman {
    table: Table,
    tree: BinaryTree,
}

impl Huffman {
    fn from_weights(weights: Lookup<Weight>) -> Self {
        let tree = BinaryTree::new(&weights);
        let encodings = tree.get_encodings();
        let decodings = BinaryTree::get_decodings(&encodings);
        let table = Table::new(weights, encodings, decodings);

        Self { table, tree }
    }

    fn learn_weights(data: &String) -> Lookup<Weight> {
        let mut weights = Lookup::<Weight>::new(0);

        for c in data.chars() {
            weights.set(&c, &(weights.lookup(&c) + 1));
        }

        weights.set(&END_CHAR, &1);

        weights
    }

    pub fn train(data: &String) -> Self {
        let weights = Self::learn_weights(data);
        Self::from_weights(weights)
    }

    pub fn check_encode(data: &String) -> Option<String> {
        Encoder::check(data)
    }

    pub fn check_decode(data: &Vec<u8>) -> Option<String> {
        Decoder::check(data)
    }

    pub fn encode(data: &String) -> Result<Vec<u8>, String> {
        match Encoder::check(data) {
            Some(err) => return Err(err),
            None => {}
        };

        let mut encoder = Encoder::new();

        let weights = Self::learn_weights(&data);
        let huffman = Self::from_weights(weights);

        encoder.write_signature();
        encoder.write_version();
        encoder.write_weights(&huffman.table.weights);
        encoder.write_string(&huffman.table.encodings, data)?;

        Ok(encoder.data)
    }

    pub fn decode(data: Vec<u8>) -> Result<String, String> {
        match Decoder::check(&data) {
            Some(err) => return Err(err),
            None => {}
        };

        let decoder = Decoder::new(data.to_vec());
        let (weights, head) = decoder.read_weights()?;
        let huffman = Huffman::from_weights(weights);

        decoder.read_body(&huffman.table.decodings, head)
    }

    pub fn get_tree(&self) -> Result<String, String> {
        self.tree.serialise()
    }

    pub fn get_table(&self) -> Result<Vec<(char, Weight, String)>, String> {
        Ok(self
            .table
            .to_vec()?
            .iter()
            .map(|(char, weight, small_code)| {
                (*char, *weight, small_code.get_string())
            })
            .collect())
    }
}
