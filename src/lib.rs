pub mod huffman;

pub mod bitstream {
    pub mod decode_stream;
    pub mod encode_stream;
}
pub mod file {
    pub mod constant;
    pub mod decoder;
    pub mod encoder;
}

pub mod table {
    pub mod encoding;
    pub mod lookup;
    pub mod table;
    pub mod weight;
}
pub mod tree {
    pub mod internal;
    pub mod leaf;
    pub mod node;
    pub mod tree;
}

// use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use crate::huffman::Huffman;

#[wasm_bindgen]
pub fn huff_encode(data: Vec<u8>) -> Result<Vec<u8>, String> {
    let string = match String::from_utf8(data) {
        Ok(string) => string,
        Err(_) => return Err("Failed to load file as utf-8".to_string()),
    };
    Huffman::encode(string)
}

#[wasm_bindgen]
pub fn huff_decode(data: Vec<u8>) -> Result<Vec<u8>, String> {
    let result = Huffman::decode(data)?;
    Ok(result.to_owned().into_bytes())
}

#[wasm_bindgen]
pub fn get_tree(data: Vec<u8>) -> String {
    let string = String::from_utf8(data).unwrap();
    Huffman::train(string).get_tree()
}

#[wasm_bindgen]
pub fn check_encode(data: Vec<u8>) -> Result<(), String> {
    let string = match String::from_utf8(data) {
        Ok(str) => str,
        Err(err) => return Err(err.to_string()),
    };
    file::encoder::Encoder::check_length(&string)
}

#[wasm_bindgen]
pub fn check_decode(data: Vec<u8>) -> Result<(), String> {
    file::decoder::Decoder::check_length(&data)?;
    file::decoder::Decoder::check_signature(&data)?;
    file::decoder::Decoder::check_version(&data)?;
    Ok(())
}
