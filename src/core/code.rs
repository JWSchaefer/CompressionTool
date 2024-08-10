#[derive(Clone, Copy, Hash, Eq)]
pub struct Code {
    raw: u32,
}

impl PartialEq for Code {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl Code {
    pub fn from_u32(raw: u32) -> Self {
        Self { raw }
    }

    pub fn new() -> Self {
        Self { raw: 1 }
    }

    pub fn get_raw(&mut self) -> u32 {
        self.raw
    }

    // pub fn set_raw(&mut self, value: &u32) {
    //     self.raw = *value
    // }

    pub fn branch_left(&mut self) {
        self.raw = self.raw << 1;
    }

    pub fn branch_right(&mut self) {
        self.raw = self.raw << 1 | 0b1;
    }

    pub fn get_string(&self) -> String {
        let string = format!("{:032b}", self.raw);

        let slice = &string[self.raw.leading_zeros() as usize + 1..];

        return slice.to_string();
    }
}
