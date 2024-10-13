pub mod huffman;
mod core {

    pub mod tree {
        mod internal;
        mod leaf;
        mod node;
        pub mod tree;
    }

    mod bitstream {
        pub mod decode_stream;
        pub mod encode_stream;
    }

    pub mod code {
        pub mod code;
        pub mod simple_code;
        pub mod small_code;
    }

    pub mod constant;
    pub mod lookup;
    pub mod table;
    pub mod weight;

    pub mod decoder;
    pub mod encoder;
}
