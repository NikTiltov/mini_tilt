#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub row: usize,
    pub cln: usize,
}

impl Cursor {
    pub fn new(row: usize, cln: usize) -> Self {
        Self { row, cln }
    }

    pub fn shift(self, row: isize, cln: isize) -> Self {
        Self {
            row: (self.row as isize + row) as usize,
            cln: (self.cln as isize + cln) as usize,
        }
    }
}

impl From<Cursor> for (u16, u16) {
    fn from(cursor: Cursor) -> Self {
        (cursor.row as u16, cursor.cln as u16)
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self { row: 0, cln: 0 }
    }
}
