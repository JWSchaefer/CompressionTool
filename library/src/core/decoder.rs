use std::collections::HashMap;
use std::iter::zip;

use super::code::Code;
use super::lookup::Lookup;
use super::weight::Weight;

use super::bitstream::decode_stream::DecodeStream;
use super::constant::{END_CHAR, SIGNATURE, SPACER, VERSION};

pub struct Decoder {
    data: Vec<u8>,
}

impl Decoder {
    pub fn new(data: Vec<u8>) -> Result<Self, String> {
        Decoder::check_length(&data)?;
        Decoder::check_signature(&data)?;
        Decoder::check_version(&data)?;

        Ok(Self { data })
    }

    pub fn check_length(data: &Vec<u8>) -> Result<(), String> {
        match data.len() == 0 {
            true => Err("Your file is empty!".to_string()),
            false => Ok(()),
        }
    }

    pub fn check_signature(data: &Vec<u8>) -> Result<(), String> {
        let sig = &data[0..SIGNATURE.len()];
        if !zip(SIGNATURE, sig).all(|(a, b)| a == b) {
            return Err("Your file isn't a valid huffman file.".to_string());
        };
        Ok(())
    }

    pub fn check_version(data: &Vec<u8>) -> Result<(), String> {
        let head = SIGNATURE.len();
        let version_len = VERSION.as_bytes().len();
        let version = &data[head..head + version_len];
        if !zip(VERSION.as_bytes(), version).all(|(a, b)| a == b) {
            return Err("Your file is the the wrong version.".to_string());
        }
        Ok(())
    }

    pub fn read_weights(&self) -> Result<(Lookup<Weight>, usize), String> {
        let mut head = SIGNATURE.len() + VERSION.as_bytes().len();

        let mut weights = Lookup::<Weight>::new(0);

        const CHAR_WIDTH: usize = (u16::BITS / 8) as usize;
        const WEIGHT_WIDTH: usize = (u32::BITS / 8) as usize;

        loop {
            // Check for End
            if !self.data[head..head + CHAR_WIDTH]
                .iter()
                .zip(&[SPACER; CHAR_WIDTH])
                .all(|(a, b)| a == b)
            {
                // Convert slice to array for u16
                let char_array: [u8; 2] =
                    match self.data[head..head + CHAR_WIDTH].try_into() {
                        Ok(array) => array,
                        Err(_) => {
                            return Err("Failed to read a char from the table."
                                .to_string())
                        }
                    };

                // Convert to uint
                let c = char::from_u32(u16::from_le_bytes(char_array) as u32)
                    .unwrap();
                head += CHAR_WIDTH;

                // Convert slice to array for u32
                let weight_array: [u8; 4] = self.data
                    [head..head + WEIGHT_WIDTH]
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

        head += CHAR_WIDTH;

        weights.set(&END_CHAR, &1);

        Ok((weights, head))
    }

    pub fn read_body(
        self,
        decodings: &HashMap<Code, char>,
        head: usize,
    ) -> Result<String, String> {
        // File content
        let content = self.data[head..].to_vec();
        let mut bitstream = DecodeStream::new(content);

        self.decode_body(decodings, &mut bitstream)
    }

    pub fn decode_body(
        self,
        decodings: &HashMap<Code, char>,
        decodestream: &mut DecodeStream,
    ) -> Result<String, String> {
        let mut n: usize;
        let mut e: Code;
        let mut out = String::new();

        'outer: loop {
            n = 1;

            loop {
                e = Code::from_u32(decodestream.read(n)?);
                let c = decodings.get(&e);
                if c.is_some() {
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
        Ok(out)
    }
}
