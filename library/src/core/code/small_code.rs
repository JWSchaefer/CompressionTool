use super::code::Code;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct SmallCode {
    raw: u32,
}

// impl PartialEq for SmallCode {
//     fn eq(&self, other: &Self) -> bool {
//         self.raw == other.raw
//     }
// }

impl Code for SmallCode {
    fn from_u32(raw: u32) -> Self {
        Self { raw }
    }

    fn new() -> Self {
        Self { raw: 1 }
    }

    fn get_raw(&mut self) -> u32 {
        self.raw
    }

    // pub fn set_raw(&mut self, value: &u32) {
    //     self.raw = *value
    // }

    fn branch_left(&mut self) {
        self.raw = self.raw << 1;
    }

    fn branch_right(&mut self) {
        self.raw = self.raw << 1 | 0b1;
    }

    fn get_string(&self) -> String {
        let string = format!("{:032b}", self.raw);

        let slice = &string[self.raw.leading_zeros() as usize + 1..];

        return slice.to_string();
    }

    fn relevant_bits(&self) -> Result<usize, String> {
        Ok(32 - self.irrelevant_bits()?)
    }

    fn irrelevant_bits(&self) -> Result<usize, String> {
        if self.raw == 0 {
            return Err("Cannot calculate the significant bits for the raw encoding '0'".to_string());
        }
        return Ok((self.raw.leading_zeros() + 1) as usize);
    }
}
