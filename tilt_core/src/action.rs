use bitflags::bitflags;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    UNDEFINED,
    Key(Key),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key {
    pub code: KeyCode,
    pub mods: KeyMods,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    UNDEFINED,
    BACKSPACE,
    INSERT,
    DELETE,
    ENTER,
    HOME,
    END,
    ESC,
    RIGHT,
    LEFT,
    DOWN,
    UP,
    PU,
    PD,
    TAB,
    BACKTAB,
    F(u8),
    CHAR(char),
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct KeyMods: u8 {
        const NONE = 0b0000_0000;
        const SHIFT = 0b0000_0001;
        const CTRL = 0b0000_0010;
        const ALT = 0b0000_0100;
        const SUPER = 0b0000_1000;
    }
}

impl Key {
    pub fn new(code: KeyCode, mods: KeyMods) -> Self {
        Self { code, mods }
    }

    pub fn get_char(self) -> Option<char> {
        if (self.mods - (KeyMods::NONE | KeyMods::SHIFT)).is_empty() {
            if let KeyCode::CHAR(c) = self.code {
                return Some(c);
            }
        }
        None
    }
}
