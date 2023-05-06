use crate::{action::Action, cursor::Cursor, Buffer};

pub trait Interface {
    fn get_action(&mut self) -> Action;
    fn render(&mut self, buffer: &dyn Buffer, cursor: Cursor);
    fn set_message(&mut self, message: String);
}
