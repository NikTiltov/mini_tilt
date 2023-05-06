use crate::cursor::Cursor;

pub trait Buffer {
    fn save(&mut self);
    fn rows(&self) -> &Vec<String>;
    fn row(&self, idx: usize) -> &String;
    fn insert(&mut self, cursor: Cursor, char: char);
    fn remove(&mut self, cursor: Cursor);
    fn replace(&mut self, cursor: Cursor, char: char);
    fn new_line(&mut self, cursor: Cursor);
    fn merge_lines(&mut self, cursor: Cursor);
    fn remove_line(&mut self, cursor: Cursor);
    fn insert_line(&mut self, cursor: Cursor, string: String);
    fn swap_lines(&mut self, cursor: Cursor);
}
