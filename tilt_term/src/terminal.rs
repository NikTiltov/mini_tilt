use std::io::Write;

use crate::key_translation::translate;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use tilt_core::{action::Action, Buffer, Cursor, Interface};

pub struct Terminal {
    stdout: std::io::Stdout,
    width: u16,
    height: u16,
    message: String,
}

impl Terminal {
    pub fn new() -> Self {
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();
        terminal::enable_raw_mode().unwrap();
        let (width, height) = terminal::size().unwrap();
        Self {
            stdout,
            width,
            height,
            message: String::new(),
        }
    }

    fn read(&self) -> Event {
        loop {
            if let Ok(Event::Key(key)) = event::read() {
                return Event::Key(key);
            }
        }
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        execute!(self.stdout, LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

impl Interface for Terminal {
    fn get_action(&mut self) -> Action {
        translate(self.read())
    }

    fn render(&mut self, buffer: &dyn Buffer, cursor: Cursor) {
        execute!(self.stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        for row in buffer.rows() {
            print!("{}\r\n", row);
        }
        let (row, cln) = cursor.into();
        execute!(self.stdout, MoveTo(0, self.height)).unwrap();
        print!("{}", self.message);
        execute!(self.stdout, MoveTo(cln, row)).unwrap();
        self.stdout.flush().unwrap();
    }

    fn set_message(&mut self, message: String) {
        self.message = message;
    }
}
