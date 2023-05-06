use crossterm::event::{Event, KeyCode as CTKeyCode, KeyEvent, KeyModifiers};
use tilt_core::action::*;

pub fn translate(event: Event) -> Action {
    match event {
        Event::Key(KeyEvent {
            code,
            modifiers,
            kind: _,
            state: _,
        }) => Action::Key(Key {
            code: translate_code(code),
            mods: translate_mods(modifiers),
        }),
        _ => Action::UNDEFINED,
    }
}

fn translate_code(code: CTKeyCode) -> KeyCode {
    use CTKeyCode::*;
    match code {
        Esc => KeyCode::ESC,
        Backspace => KeyCode::BACKSPACE,
        Enter => KeyCode::ENTER,
        Left => KeyCode::LEFT,
        Right => KeyCode::RIGHT,
        Up => KeyCode::UP,
        Down => KeyCode::DOWN,
        Home => KeyCode::HOME,
        End => KeyCode::END,
        PageUp => KeyCode::PU,
        PageDown => KeyCode::PD,
        Tab => KeyCode::TAB,
        BackTab => KeyCode::BACKTAB,
        Delete => KeyCode::DELETE,
        Insert => KeyCode::INSERT,
        F(num) => KeyCode::F(num),
        Char(sym) => KeyCode::CHAR(sym),
        _ => KeyCode::UNDEFINED,
    }
}

fn translate_mods(modifiers: KeyModifiers) -> KeyMods {
    KeyMods::from_bits_truncate(modifiers.bits())
}
