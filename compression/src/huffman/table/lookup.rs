pub const MAX_CHAR : usize = 20_000;

pub struct Lookup<T>{
    data : [T; MAX_CHAR]
}

impl<T: Copy> Lookup<T> {

    pub fn new(fill: T) -> Self {
        Lookup {
            data: [fill; MAX_CHAR],
        }
    }

    pub fn lookup(&self, index : &char) -> T {
        self.data[*index as usize]
    }

    pub fn set(&mut self, index : &char, value : &T) {
        self.data[*index as usize] = *value
    }

}