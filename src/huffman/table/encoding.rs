pub trait HuffEncoding {

    fn new() -> Self;

    fn branch_left(&mut self);
    fn branch_right(&mut self);

    fn get_raw(&mut self) -> u32;
    fn set_raw(&mut self, value : &u32);

}

#[derive(Clone, Copy, Hash, Eq)]
pub struct Encoding{
    raw : u32
}

impl Encoding {
    pub fn from_u32(raw : u32) -> Self {
        Self {raw}
    }

}

impl PartialEq for Encoding {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl HuffEncoding for Encoding {

    fn new() -> Self {
        Self {raw : 1}
    }

    fn get_raw(&mut self) -> u32 {
        self.raw
    }

    fn set_raw(&mut self, value : &u32) {
        self.raw = *value
    }

    fn branch_left(&mut self) {
        self.raw = self.raw << 1;
    }

    fn branch_right(&mut self) {
        self.raw = self.raw << 1 | 0b1;
    }

}