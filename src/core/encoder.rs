use super::constant::{END_CHAR, MAX_CHAR, SIGNATURE, SPACER, VERSION};

use super::bitstream::encode_stream::EncodeStream;
use super::code::Code;
use super::lookup::Lookup;
use super::weight::Weight;
pub struct Encoder {
    pub data: Vec<u8>,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            data: Vec::<u8>::new(),
        }
    }

    pub fn check_length(data: &String) -> Result<(), String> {
        match data.len() == 0 {
            true => Err("File is empty!".to_string()),
            false => Ok(()),
        }
    }

    pub fn write_signature(&mut self) {
        self.data.append(&mut Vec::from(SIGNATURE));
    }

    pub fn write_version(&mut self) {
        self.data.append(&mut Vec::from(VERSION.as_bytes()));
    }

    pub fn write_weights(&mut self, weights: &Lookup<Weight>) {
        let (mut w, mut row): (u32, Vec<u8>);

        for c in 1..MAX_CHAR as u16 {
            w = weights.lookup(&c);

            if w != 0 {
                row = Vec::from(c.to_le_bytes());
                row.append(&mut Vec::from(w.to_le_bytes()));
                self.data.append(&mut row)
            }
        }

        self.data.append(&mut Vec::from([SPACER; 2]));
    }

    pub fn write_string(&mut self, encodings: &Lookup<Code>, data: &String) {
        let mut encoding: Code;
        let mut stream = EncodeStream::new(Vec::<u8>::new());

        for c in data.chars() {
            encoding = encodings.lookup(&c);
            stream.put(&mut encoding.get_raw());
        }

        stream.put(&mut encodings.lookup(&END_CHAR).get_raw());

        self.data.append(&mut stream.get_data());
    }
}
