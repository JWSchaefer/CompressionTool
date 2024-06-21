pub const MAX_CHAR : usize = 20_000 as usize;

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
    data : [T; MAX_CHAR]
}

impl<T> Lookup<T>
where
    T: Copy + PartialEq,
{

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T>{
        self.data.iter_mut()
    }

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

    pub fn search(&self, value: &T) -> Option<usize> {

        for (i, v) in self.data.iter().enumerate() {
            if v == value {
                return Some(i);
            } else {
                continue;
            }
        }

        None
    }

}