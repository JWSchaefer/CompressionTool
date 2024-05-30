use std::collections::HashMap;
use itertools::Itertools;

use crate::huffman::leaf::Leaf;
use crate::huffman::internal::Internal;
use crate::huffman::node::Node;
use crate::huffman::types::Weight;

pub struct Tree{
    root : Box<dyn Node>,
}

impl Tree{

    pub fn new(data : &String) -> Self {

        // Instantiate empty hashmap from char <-> frequency 
        let mut map: HashMap<char, u32> = HashMap::new();

        // Iterate through string and count occurences 
        for c in data.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        // Map to sorted vector of n leaves
        let mut nodes: Vec<Box<dyn Node>> = map.iter()
        .map(|(&char, &weight)| Box::new(Leaf::new(char, weight)) as Box<dyn Node>)
        .collect();

        // Sort by frequency
        nodes.sort_by(|a, b| b.get_weight().cmp(&a.get_weight()));

        // Build tree
        while nodes.len() > 2 {

            // Pop two smallest 
            let l : Box<dyn Node> = nodes.pop().unwrap();
            let r : Box<dyn Node> = nodes.pop().unwrap();

            // Create new internal node
            let internal = Box::new(Internal::new(l, r));

            // Build weights vector
            let weights : Vec<Weight> = nodes
            .iter().map(|node| node.get_weight())
            .collect();

            // // Identify index to insert new internal node
            // let insertion = match weights.iter()
            // .find_position(|&&x| x <= internal.get_weight()) {
            //     None => 0,
            //     Some((a , _)) => a
            // };

            // Identify index to insert new internal node
            let insertion = match weights.iter()
            .find_position(|&&x| x <= internal.get_weight()) {
                None => 0,
                Some((a , _)) => a
            };

            nodes.insert(insertion, internal);

        }

        let root = nodes.pop().unwrap();

        Tree {root}
    }
}