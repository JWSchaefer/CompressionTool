use std::iter::zip;
use std::collections::HashMap;

use crate::huffman;

use super::super::table::table::Table;
use super::super::table::weight::Weight;
use super::super::tree::tree::BinaryTree;
use super::super::bitstream::decode_stream::DecodeStream;
use super::super::bitstream::encode_stream::EncodeStream;
use super::super::table::encoding::{Encoding, HuffEncoding};
use super::super::table::lookup::{Lookup, MAX_CHAR, Indexable};

use super::constant::{END_CHAR, SIGNATURE, SPACER, VERSION};

pub struct Huffman {
    table : Table,
    tree  : BinaryTree
}

impl Huffman {

    pub fn train(data : &String) -> Self {

        let mut weights = Lookup::<Weight>::new(0);

        for c in data.chars() {
            weights.set(&c, &(weights.lookup(&c) + 1));
        }

        weights.set(&END_CHAR, &1);

        let tree = BinaryTree::new(&weights);
        
        let encodings = tree.get_encodings();

        let decodings = BinaryTree::get_decodings(&encodings);

        let table = Table::new(weights, encodings, decodings);

        Self { table, tree }
    }



    pub fn encode(data : &mut String) -> Vec<u8> {

        let huf = Self::train(data);

        // File Signature
        let mut buf = Vec::from(SIGNATURE);

        // Version
        buf.append(
            &mut Vec::from(
                VERSION
                .as_bytes())
        );

        // Weights
        buf.append(&mut huf.encode_table() );
    
        // Encode File data
        let mut stream = EncodeStream::new(Vec::<u8>::new());
        let mut encoding : Encoding;
        data.push(END_CHAR);

        for c in data.chars() {
            encoding = huf.table.encodings.lookup(&c);
            stream.put(&mut encoding.get_raw());
        }

        // Write File Data
        buf.append(&mut stream.get_data());

        buf
    }

    fn encode_table(&self) -> Vec<u8> {
        let mut buf = Vec::<u8>::new();

        let (mut w, mut row) : (u32, Vec<u8>);

        for c in 1..MAX_CHAR as u16 {
            
            w = self.table.weights.lookup(&c);

            if w != 0 {
                row = Vec::from(c.to_le_bytes());
                row.append(&mut Vec::from(w.to_le_bytes()));
                buf.append(&mut row)
            }
        }

        buf.append(&mut Vec::from([SPACER; 2]));

        buf
    }

    pub fn decode(data : &Vec<u8>) -> Result<String, &str> {

        // Weights
        let (mut huffman, mut head) = Self::train_raw(&data)?;

        // File content
        let mut content = data[head..].to_vec();
        let mut bitstream = DecodeStream::new(content);

        let mut res = String::new();
        Ok(huffman.decode_body(bitstream, res))
       
    }

    fn decode_body(
        &mut self, 
        mut decodestream : DecodeStream, 
        mut out : String
    ) -> String {

        let mut c : usize;
        let mut n : usize;
        let mut w : Weight;
        let mut e : Encoding;
        let mut i : Option<usize>;

        'outer : loop {
            
            n = 1;
            e = Encoding::new();

            loop {
                e = Encoding::from_u32(decodestream.read(n));
                let c =  self.table.decodings.get(&e);
                if c.is_some(){
                    out.push(*c.unwrap());
                    decodestream.discard(n);
                    if *c.unwrap() == 0 as char {
                        break 'outer;
                    } 
                    break;
                } else {
                    n += 1;
                }
            }
        }
        out.pop();
        out
    }

    fn train_raw(data : &Vec<u8>) -> Result<(Self, usize), &str> {

        let mut head = 0;

        // File Signature
        let sig = &data[head..SIGNATURE.len()];

        if !zip(SIGNATURE, sig).all(|(a,b)| a == b) {
            return Err("Invalid File Signature");
        } 

        head += SIGNATURE.len();

        // Version
        let version_len = VERSION
        .as_bytes()
        .len();

        let version = &data[head..head + version_len];

        if !zip(VERSION.as_bytes(), version).all(|(a,b)| a == b) {
            return Err("File and decoder versions do not match, unable to decode.")
        } 

        head += version_len; 

        let mut weights = Lookup::<Weight>::new(0);
        
        const CHAR_WIDTH   : usize = (u16::BITS / 8) as usize;
        const WEIGHT_WIDTH : usize = (u32::BITS / 8) as usize;

        loop {

            // Check for End
            if !data[head..head + CHAR_WIDTH]
                .iter()
                .zip(&[SPACER; CHAR_WIDTH])
                .all(|(a, b)| a == b)
            {

                // Convert slice to array for u16
                let char_array : [u8; 2] = data[head..head + CHAR_WIDTH]
                .try_into()
                .expect("Failed to read char from table.");
                
                // Convert to uint
                let c = char::from_u32(u16::from_le_bytes(char_array) as u32)
                .unwrap();
                head += CHAR_WIDTH;

                // Convert slice to array for u32
                let weight_array: [u8; 4] = data[head..head + WEIGHT_WIDTH]
                .try_into()
                .expect("Failed to read weight from table.");

                // Convert to uint
                let w = u32::from_le_bytes(weight_array);
                head += WEIGHT_WIDTH;

                // Update weights table
                weights.set(&c, &w)

            } else {
                break;
            }
        }

        weights.set(&END_CHAR, &1);

        let tree = BinaryTree::new(&weights);

        let encodings = tree.get_encodings();

        let decodings = BinaryTree::get_decodings(&encodings);

        let table = Table::new(weights, encodings, decodings);

        head += CHAR_WIDTH;

        Ok((Self { table, tree }, head))

    }
}


 