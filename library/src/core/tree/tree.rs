use std::collections::HashMap;

use itertools::Itertools;

use super::internal::Internal;
use super::leaf::Leaf;
use super::node::Node;

use super::super::code::{code::Code, small_code::SmallCode};
use super::super::constant::MAX_CHAR;
use super::super::lookup::Lookup;
use super::super::weight::Weight;

pub struct BinaryTree {
    root: Box<dyn Node>,
}

impl BinaryTree {
    pub fn new(weights: &Lookup<Weight>) -> Self {
        // Create node vector
        let mut nodes: Vec<Box<dyn Node>> = Vec::new();

        // Count non-zero leaves and initialise
        for c in 0..MAX_CHAR as u32 {
            // Skip invalid chars
            if (0xD800..=0xDFFF).contains(&c) {
                continue;
            }

            let msg = format!("{c} could not be converted to char");
            let c = char::from_u32(c).expect(&msg);

            match weights.lookup(&c) {
                0 => {}
                other => {
                    nodes.push(Box::new(Leaf::new(c, other)) as Box<dyn Node>);
                }
            }
        }

        // Sort by frequency
        nodes.sort_by(|a, b| b.get_weight().cmp(&a.get_weight()));

        // Build tree
        while nodes.len() > 1 {
            // Pop two smallest
            let l: Box<dyn Node> = nodes.pop().unwrap();
            let r: Box<dyn Node> = nodes.pop().unwrap();

            // Create new internal node
            let internal = Box::new(Internal::new(l, r));

            // Build weights vector
            let w: Vec<Weight> =
                nodes.iter().map(|node| node.get_weight()).collect();

            // Identify index to insert new internal node
            let insertion = match w
                .iter()
                .find_position(|&&x| x <= internal.get_weight())
            {
                None => 0,
                Some((a, _)) => a,
            };

            nodes.insert(insertion, internal);
        }

        let root = nodes.pop().unwrap();

        Self { root }
    }

    pub fn serialise(&self) -> Result<String, String> {
        Err("Not implemented.".to_string())
    }

    pub fn get_decodings(
        encodings: &Lookup<SmallCode>,
    ) -> HashMap<SmallCode, char> {
        let mut decodings = HashMap::<SmallCode, char>::new();

        for _c in 0..MAX_CHAR {
            // Skip invalid chars
            if (0xD800..=0xDFFF).contains(&_c) {
                continue;
            }

            let c = char::from_u32(_c as u32).unwrap();
            let mut e = encodings.lookup(&c);
            if e.get_raw() != 1 {
                decodings.insert(e, c);
            }
        }

        decodings
    }

    pub fn get_encodings(&self) -> Lookup<SmallCode> {
        let mut encodings = Lookup::<SmallCode>::new(SmallCode::new());

        let mut state = SmallCode::new();

        Self::_get_encodings(&self.root, &mut state, &mut encodings);

        encodings
    }

    pub fn _get_encodings(
        node: &Box<dyn Node>,
        state: &mut SmallCode,
        encodings: &mut Lookup<SmallCode>,
    ) {
        match node.get_char() {
            Some(c) => {
                encodings.set(&c, &state);
            }
            None => {}
        }

        match node.get_children() {
            Some((left, right)) => {
                let mut state0 = state.clone();
                let mut state1 = state.clone();

                state0.branch_left();
                state1.branch_right();

                Self::_get_encodings(&left, &mut state0, encodings);
                Self::_get_encodings(&right, &mut state1, encodings);
            }
            None => {}
        }
    }
}
