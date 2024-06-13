// pub type Encoding = u32;

pub trait HuffEncoding {

    fn new() -> Self;

    fn put_left(&mut self);
    fn put_right(&mut self);

    fn get_raw(&mut self) -> u32;
    fn set_raw(&mut self, value : &u32);

}

#[derive(Clone, Copy)]
pub struct Encoding{
    raw : u32
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

    fn put_left(&mut self) {
        self.raw = self.raw << 1;
    }

    fn put_right(&mut self) {
        self.raw = self.raw << 1 | 0b1;
    }

}