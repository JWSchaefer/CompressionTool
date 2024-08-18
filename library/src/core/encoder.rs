use super::constant::{END_CHAR, MAX_CHAR, SIGNATURE, SPACER, VERSION};

use super::bitstream::encode_stream::EncodeStream;
use super::code::small_code::SmallCode;
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

    pub fn check(data: &String) -> Option<String> {
        if !Encoder::check_length(&data) {
            return Some("Cannot encode an empty file.".to_string());
        }
        None
    }

    pub fn check_length(data: &String) -> bool {
        data.len() != 0
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

    pub fn write_string(
        &mut self,
        encodings: &Lookup<SmallCode>,
        data: &String,
    ) -> Result<(), String> {
        let mut encoding: SmallCode;
        let mut stream = EncodeStream::new(Vec::<u8>::new());

        for c in data.chars() {
            encoding = encodings.lookup(&c);
            stream.put(&mut encoding)?;
        }

        stream.put(&mut encodings.lookup(&END_CHAR))?;

        self.data.append(&mut stream.get_data());

        Ok(())
    }
}
