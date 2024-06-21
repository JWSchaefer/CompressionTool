use std::result;
use colored::Colorize;
pub struct DecodeStream {
    data : Vec::<u8>,
    bit_head  : usize,
    byt_head : usize,

}

impl DecodeStream {
    
    pub fn new(mut data : Vec<u8>) -> Self {
        Self { data : data, bit_head : 7, byt_head : 0}
    }

    pub fn get_data(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    pub fn read(&self, n_bits : usize) -> u32{

        let bits = (7 - self.bit_head) + n_bits;

        let bytes = (bits / 8) + (bits % 8 != 0) as usize;

        let len = self.data.len() - 1;

        if bytes > 4{
            panic!("Attempting to read more than 4 bytes from the stream")
        }

        let mut result : [u8; 4] = [0;4];
        result[0..bytes].copy_from_slice(&self.data[self.byt_head..self.byt_head+bytes]);

        for i in result {
            i.reverse_bits();
        }

        let mut result = u32::from_be_bytes(result);

        result >>= 32 - bits;

        result |= 0b1 << n_bits;    

        result 
    }


    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn discard(&mut self, n_bits : usize){

        let bits = (7 - self.bit_head) + n_bits;

        let bytes = (bits/8) - 1 + (bits % 8 != 0) as usize;

        self.bit_head = 7 - (bits % 8);

        self.byt_head += bytes;

        self.data[self.byt_head] &= !(u8::MAX << self.bit_head+1);
        
        if self.bit_head == 7{
            self.byt_head += 1;
        }

    }
}

