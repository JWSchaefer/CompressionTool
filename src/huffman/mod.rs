
mod tree {
    pub mod tree;
    pub mod node;
    pub mod leaf;
    pub mod internal;

}
mod table {
    pub mod table;
    pub mod weight;
    pub mod lookup;
    pub mod encoding;
}

mod bitstream {
    pub mod decode_stream;
    pub mod encode_stream;
}

mod file {    
    pub mod encoder;
    pub mod decoder;
    pub mod constant;
}

pub mod huffman;