pub trait Code {
    fn new() -> Self;
    fn from_u32(raw: u32) -> Self;
    fn get_raw(&mut self) -> u32;
    fn branch_left(&mut self);
    fn branch_right(&mut self);
    fn get_string(&self) -> String;
    fn relevant_bits(&self) -> Result<usize, String>;
    fn irrelevant_bits(&self) -> Result<usize, String>;
}
