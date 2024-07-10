pub const MAX_CHAR: usize = 20_000 as usize;

pub trait Indexable {
    fn to_usize(self) -> usize;
}

impl Indexable for usize {
    fn to_usize(self) -> usize {
        self
    }
}

impl Indexable for char {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Indexable for u16 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Indexable for u32 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

pub struct Lookup<T> {
    data: [T; MAX_CHAR],
}

impl<T> Lookup<T>
where
    T: Copy + PartialEq,
{
    pub fn new(fill: T) -> Self {
        Lookup {
            data: [fill; MAX_CHAR],
        }
    }

    pub fn lookup<I>(&self, index: &I) -> T
    where
        I: Indexable + Copy,
    {
        self.data[(*index).to_usize()]
    }

    pub fn set<I>(&mut self, index: &I, value: &T)
    where
        I: Indexable + Copy,
    {
        self.data[(*index).to_usize()] = *value;
    }
}
