use super::super::table::weight::Weight;

pub trait Node {
    fn get_char(&self) -> Option<char>;
    fn get_weight(&self) -> Weight;
    fn get_children(&self) -> Option<(&Box<dyn Node>, &Box<dyn Node>)>;
}
