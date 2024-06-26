use std::iter::zip;
use std::collections::HashMap;

use super::constant::{VERSION, SIGNATURE, SPACER, END_CHAR};
use super::super::table::lookup::Lookup;
use super::super::table::weight::Weight;
use super::super::table::encoding::Encoding;
use super::super::bitstream::decode_stream::DecodeStream;


pub struct Decoder {
    data : Vec<u8>
}

impl Decoder {

    pub fn new( data : Vec<u8> ) -> Self {
        let decoder = Self { data };
        decoder.check_signature();
        decoder.check_version();
        decoder
    }   

    fn check_signature(&self) {
        let sig = &self.data[0..SIGNATURE.len()];
        if !zip(SIGNATURE, sig).all(|(a,b)| a == b){
            panic!("Input is lacking the required file signature.");
        };
    }

    fn check_version(&self) {
        let head        = SIGNATURE.len();
        let version_len = VERSION.as_bytes().len();
        let version     = &self.data[head..head + version_len];
        if ! zip(VERSION.as_bytes(), version).all(|(a,b)| a == b){
            panic!("This encoding was generated by a incompatible version.");
        }
    }

    pub fn read_weights(&self) -> (Lookup<Weight>, usize) {

        let mut head = SIGNATURE.len() + VERSION.as_bytes().len();

        let mut weights = Lookup::<Weight>::new(0);
        
        const CHAR_WIDTH   : usize = (u16::BITS / 8) as usize;
        const WEIGHT_WIDTH : usize = (u32::BITS / 8) as usize;

        loop {

            // Check for End
            if !self.data[head..head + CHAR_WIDTH]
                .iter()
                .zip(&[SPACER; CHAR_WIDTH])
                .all(|(a, b)| a == b)
            {

                // Convert slice to array for u16
                let char_array : [u8; 2] = self.data[head..head + CHAR_WIDTH]
                .try_into()
                .expect("Failed to read char from table.");
                
                // Convert to uint
                let c = char::from_u32(u16::from_le_bytes(char_array) as u32)
                .unwrap();
                head += CHAR_WIDTH;

                // Convert slice to array for u32
                let weight_array: [u8; 4] = self.data[head..head + WEIGHT_WIDTH]
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

        (weights, head)

    }

    pub fn read_body(self, decodings : &HashMap<Encoding, char> , head : usize) -> String {

        // File content
        let content = self.data[head..].to_vec();
        let mut bitstream = DecodeStream::new(content);

        self.decode_body(decodings, &mut bitstream)

    }


    
    pub fn decode_body(self, decodings : &HashMap<Encoding, char>, decodestream : &mut DecodeStream) -> String {

        let mut n : usize;
        let mut e : Encoding;
        let mut out = String::new();

        'outer : loop {
            
            n = 1;

            loop {
                e = Encoding::from_u32(decodestream.read(n));   
                let c = decodings.get(&e);
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


}