use crate::{buffer::Buffer, cursor::Cursor};

pub struct SimpleBuffer {
    rows: Vec<String>,
    path: Option<String>,
}

impl SimpleBuffer {
    pub fn new() -> Self {
        Self {
            rows: vec![String::new()],
            path: None,
        }
    }
}

impl Buffer for SimpleBuffer {
    fn save(&mut self) {
        todo!()
    }

    fn rows(&self) -> &Vec<String> {
        &self.rows
    }

    fn row(&self, idx: usize) -> &String {
        &self.rows[idx]
    }

    fn insert(&mut self, cursor: Cursor, char: char) {
        self.rows[cursor.row].insert(cursor.cln, char);
    }

    fn remove(&mut self, cursor: Cursor) {
        self.rows[cursor.row].remove(cursor.cln - 1);
    }

    fn replace(&mut self, cursor: Cursor, char: char) {
        self.remove(cursor);
        self.insert(cursor, char);
    }

    fn new_line(&mut self, cursor: Cursor) {
        let before: String = self.rows[cursor.row].chars().take(cursor.cln).collect();
        let after: String = self.rows[cursor.row].chars().skip(cursor.cln).collect();
        self.rows[cursor.row] = before;
        self.rows.insert(cursor.row + 1, after);
    }

    fn merge_lines(&mut self, cursor: Cursor) {
        self.rows[cursor.row] = self.rows[cursor.row].clone() + &self.rows[cursor.row + 1];
        self.remove_line(cursor.shift(1, 0));
    }

    fn remove_line(&mut self, cursor: Cursor) {
        self.rows.remove(cursor.row);
        if self.rows.is_empty() {
            self.rows.push(String::new());
        }
    }

    fn insert_line(&mut self, cursor: Cursor, string: String) {
        self.rows.insert(cursor.row, string);
    }

    fn swap_lines(&mut self, cursor: Cursor) {
        self.rows.swap(cursor.row, cursor.row + 1);
    }
}
