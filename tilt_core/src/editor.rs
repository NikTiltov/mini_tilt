use crate::{
    buffer::Buffer,
    cursor::Cursor,
    event::{EditorEvent, Move},
    simple_buffer::SimpleBuffer,
};

pub struct Editor {
    buffer: Box<dyn Buffer>,
    cursor: Cursor,
}

impl Editor {
    pub fn new(buffer: Option<Box<dyn Buffer>>) -> Self {
        Self {
            buffer: buffer.unwrap_or(Box::new(SimpleBuffer::new())),
            cursor: Cursor::default(),
        }
    }

    pub fn buffer(&self) -> &dyn Buffer {
        self.buffer.as_ref()
    }

    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    fn cur_row(&mut self) -> &String {
        self.buffer.row(self.cursor.row)
    }

    fn to_line_begin(&mut self) {
        self.cursor.cln = 0;
    }

    fn to_line_end(&mut self) {
        self.cursor.cln = self.cur_row().len();
    }

    fn clns_fix(&mut self) {
        if self.cursor.cln > self.cur_row().len() {
            self.to_line_end();
        }
    }

    pub fn execute(&mut self, event: EditorEvent) {
        match event {
            EditorEvent::Insert(char) => {
                self.buffer.insert(self.cursor, char);
                self.cursor.cln += 1;
            }
            EditorEvent::NewLine => {
                self.buffer.new_line(self.cursor);
                self.cursor.row += 1;
                self.to_line_begin();
            }
            EditorEvent::CursorMove(movement) => match movement {
                Move::Left => {
                    if self.cursor.cln > 0 {
                        self.cursor.cln -= 1;
                    } else if self.cursor.row > 0 {
                        self.cursor.row -= 1;
                        self.to_line_end();
                    }
                }
                Move::Right => {
                    if self.cursor.cln < self.cur_row().len() {
                        self.cursor.cln += 1;
                    } else if self.cursor.row + 1 < self.buffer.rows().len() {
                        self.cursor.row += 1;
                        self.to_line_begin();
                    }
                }
                Move::Down => {
                    if self.cursor.row + 1 < self.buffer.rows().len() {
                        self.cursor.row += 1;
                        self.clns_fix();
                    }
                }
                Move::Up => {
                    if self.cursor.row > 0 {
                        self.cursor.row -= 1;
                        self.clns_fix();
                    }
                }
                Move::ToFileBegin => {
                    self.cursor = Cursor::new(0, 0);
                }
                Move::ToFileEnd => {
                    self.cursor.row = self.buffer.rows().len() - 1;
                    self.to_line_end();
                }
                Move::ToLineBegin => {
                    self.to_line_begin();
                }
                Move::ToLineEnd => {
                    self.to_line_end();
                }
            },
            EditorEvent::Remove => {
                if self.cursor.cln == 0 {
                    if self.cursor.row != 0 {
                        self.cursor.row -= 1;
                        self.to_line_end();
                        self.buffer.merge_lines(self.cursor);
                    }
                } else {
                    self.buffer.remove(self.cursor);
                    self.cursor.cln -= 1;
                }
            }
            EditorEvent::Delete => {
                if self.cursor.cln == self.cur_row().len() {
                    if self.cursor.row + 1 != self.buffer.rows().len() {
                        self.buffer.merge_lines(self.cursor);
                    }
                } else {
                    self.buffer.remove(self.cursor.shift(0, 1));
                }
            }
            EditorEvent::AddTab => {
                for _ in 0..(4 - self.cursor.cln % 4) {
                    self.buffer.insert(self.cursor, ' ');
                    self.cursor.cln += 1;
                }
            }
            EditorEvent::RemoveTab => {
                let count = self
                    .cur_row()
                    .chars()
                    .take_while(|&char| char == ' ')
                    .count();
                if count == 0 {
                    return;
                }
                for _ in 0..((count - 1) % 4 + 1) {
                    self.buffer.remove(Cursor::new(self.cursor.row, 1));
                    self.cursor.cln -= 1;
                }
            }
            EditorEvent::Copy => {}
            EditorEvent::Paste => {}
            EditorEvent::Cut => {}
            EditorEvent::CopyLineAbove => {
                let copy = self.cur_row().clone();
                self.buffer.insert_line(self.cursor, copy);
            }
            EditorEvent::CopyLineBelow => {
                let copy = self.cur_row().clone();
                self.buffer.insert_line(self.cursor, copy);
                self.cursor.row += 1;
            }
            EditorEvent::DeleteLine => {
                self.buffer.remove_line(self.cursor);
                if self.cursor.row > 0 && self.cursor.row == self.buffer.rows().len() {
                    self.cursor.row -= 1;
                }
                self.clns_fix();
            }
            EditorEvent::MoveLineUp => {
                if self.cursor.row == 0 {
                    return;
                }
                self.buffer.swap_lines(self.cursor.shift(-1, 0));
                self.cursor.row -= 1;
            }
            EditorEvent::MoveLineDown => {
                if self.cursor.row + 1 == self.buffer.rows().len() {
                    return;
                }
                self.buffer.swap_lines(self.cursor);
                self.cursor.row += 1;
            }
        }
    }
}
