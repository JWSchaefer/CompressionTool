use crate::file::constant::END_CHAR;
use crate::file::decoder::Decoder;
use crate::file::encoder::Encoder;
use crate::table::lookup::Lookup;
use crate::table::table::Table;
use crate::table::weight::Weight;
use crate::tree::tree::BinaryTree;

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

    fn gen_weights(data: &String) -> Lookup<Weight> {
        let mut weights = Lookup::<Weight>::new(0);

        for c in data.chars() {
            weights.set(&c, &(weights.lookup(&c) + 1));
        }

        weights.set(&END_CHAR, &1);

        weights
    }

    pub fn train(data: String) -> Self {
        let weights = Self::gen_weights(&data);

        Self::from_weights(weights)
    }

    pub fn encode(mut data: String) -> Result<Vec<u8>, String> {
        Encoder::check_length(&data)?;

        let weights = Self::gen_weights(&data);
        let huffman = Self::from_weights(weights);

        let mut encoder = Encoder::new();

        encoder.write_signature();
        encoder.write_version();
        encoder.write_weights(&huffman.table.weights);
        encoder.write_string(&huffman.table.encodings, &mut data);

        Ok(encoder.data)
    }

    pub fn decode(data: Vec<u8>) -> Result<String, String> {
        let decoder = Decoder::new(data.to_vec())?;
        let (weights, head) = decoder.read_weights()?;
        let huffman = Huffman::from_weights(weights);

        decoder.read_body(&huffman.table.decodings, head)
    }

    pub fn get_tree(&self) -> String {
        self.tree.serialise()
    }
}
