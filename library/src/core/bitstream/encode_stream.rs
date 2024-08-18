use super::super::code::code::Code;

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

    pub fn put<T>(&mut self, encoding: &mut T) -> Result<(), String>
    where
        T: Code,
    {
        // Determine how far to shift the encoding
        let bits: usize = encoding.relevant_bits()?;
        let diff: usize = ((32 + self.head + 1) - bits) % 8;

        let size: usize = bits + diff;

        let mut bytes_to_write: usize = size / 8;

        if size % 8 != 0 {
            bytes_to_write += 1;
        }

        // Raise an error if shifting the bit would cause an overflow
        // This is fixable but is not woth the effort right now
        if diff > encoding.irrelevant_bits()? {
            panic!("Code cannot be coerced into the stream without information loss")
        }

        let mut raw: u32 = encoding.get_raw();

        // Clear the lead bit and shift
        raw &= !(0b1 << bits);
        raw <<= diff;

        // Split the encoding into u8 bytes
        let bytes: &[u8] = &raw.to_be_bytes()[4 - bytes_to_write..];

        // Join the two mismatched bytes
        let len: usize = self.data.len() - 1;

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

        Ok(())
    }
}
