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

    pub fn lookup(&self, char : &char) -> T {
        self.data[*char as usize]
    }

    pub fn lookup_index(&self, index : &usize) -> T {
        self.data[*index]
    }

    pub fn set(&mut self, char : &char, value : &T) {
        self.data[*char as usize] = *value
    }

    // pub fn set_index(&mut self, index : &usize, value : &T) {
    //     self.data[*index] = *value
    // }

}