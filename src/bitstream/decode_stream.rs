pub struct DecodeStream {
    data: Vec<u8>,
    bit_head: usize,
    byt_head: usize,
}

impl DecodeStream {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: data,
            bit_head: 7,
            byt_head: 0,
        }
    }

    pub fn read(&self, n_bits: usize) -> Result<u32, String> {
        let bits = (7 - self.bit_head) + n_bits;

        let bytes = (bits / 8) + (bits % 8 != 0) as usize;

        // let len = self.data.len() - 1;

        if bytes > 4 {
            return Err("Attempting to read more than 4 bytes from the stream"
                .to_string());
        }

        let mut result_array: [u8; 4] = [0; 4];
        result_array[0..bytes]
            .copy_from_slice(&self.data[self.byt_head..self.byt_head + bytes]);

        let mut result = u32::from_be_bytes(result_array);

        result >>= 32 - bits;

        result |= 0b1 << n_bits;

        Ok(result)
    }

    pub fn discard(&mut self, n_bits: usize) {
        let bits = (7 - self.bit_head) + n_bits;

        let mut bytes = bits / 8;

        if bits % 8 == 0 {
            bytes -= 1;
        }

        self.bit_head = 7 - (bits % 8);
        self.byt_head += bytes;

        let mask = u8::MAX << 1;

        self.data[self.byt_head] &= !(mask << self.bit_head);

        if self.bit_head == 7 {
            self.byt_head += 1;
        }
    }
}
