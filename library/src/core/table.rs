use super::code::Code;
use super::lookup::Lookup;
use super::weight::Weight;

use super::constant::{END_CHAR, MAX_CHAR};

use std::char;
use std::collections::HashMap;
pub struct Table {
    pub weights: Lookup<Weight>,
    pub encodings: Lookup<Code>,
    pub decodings: HashMap<Code, char>,
}

impl Table {
    pub fn new(
        weights: Lookup<Weight>,
        encodings: Lookup<Code>,
        decodings: HashMap<Code, char>,
    ) -> Self {
        Self {
            weights,
            encodings,
            decodings,
        }
    }

    pub fn to_vec(&self) -> Result<Vec<(char, Weight, Code)>, String> {
        let mut output = Vec::<(char, Weight, Code)>::new();

        for c in 0..MAX_CHAR as u32 {
            // Skip invalid chars
            if (0xD800..=0xDFFF).contains(&c) {
                continue;
            }

            if c == END_CHAR as u32 {
                continue;
            }

            let weight = self.weights.lookup(&c);

            if weight == 0 {
                continue;
            }

            let c_as_char = match char::from_u32(c) {
                Some(_c) => _c,
                None => return Err(format!("Failed to convert {c} into char")),
            };

            let encoding = self.encodings.lookup(&c);

            output.push((c_as_char, weight, encoding));
        }

        Ok(output)
    }
}
