pub struct EncodeStream {
    data: Vec<u8>,
    head: usize,
}

impl EncodeStream {
    pub fn new(mut data: Vec<u8>) -> Self {
        data.push(0);
        Self {
            data: data,
            head: 7,
        }
    }

    pub fn get_data(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    pub fn put(&mut self, encoding: &mut u32) {
        // Determine how far to shift the encoding
        let front: u32 = encoding.leading_zeros() + 1;
        let bits: u32 = 32 - front;
        let diff: u32 = (((32 + self.head + 1) as u32) - bits) % 8;

        let size = (bits + diff) as usize;
        let bytes_to_write = (size / 8) + (size % 8 != 0) as usize;

        // Raise an error if shifting the bit would cause an overflow
        // This is fixable but is not woth the effort right now
        if diff > front {
            panic!("Encoding cannot be coerced into the stream without information loss")
        }

        // Clear the lead bit and shift
        *encoding &= !(0b1 << bits);
        *encoding <<= diff;

        // Split the encoding into u8 bytes
        let bytes = &encoding.to_be_bytes()[4 - bytes_to_write..];

        // Join the two mismatched bytes
        let len = self.data.len() - 1;
        self.data[len] |= bytes[0];

        if bytes_to_write > 1 {
            self.data.append(&mut bytes[1..bytes_to_write].to_vec());
        }

        match diff.checked_sub(1) {
            Some(d) => self.head = d as usize,
            None => {
                self.head = 7;
                self.data.push(0)
            }
        }
    }
}
