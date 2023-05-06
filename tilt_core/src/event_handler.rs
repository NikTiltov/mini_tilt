use crate::{
    action::{Key, KeyCode, KeyMods},
    event::{AppEvent, EditorEvent, Event, Move},
};
use std::collections::HashMap;

pub struct EventHandler {
    keybinds: HashMap<Key, Event>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            keybinds: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: Key, event: Event) {
        self.keybinds.insert(key, event);
    }

    pub fn remove(&mut self, key: &Key) {
        self.keybinds.remove(key);
    }

    pub fn handle(&self, key: &Key) -> Option<Event> {
        if let Some(c) = key.get_char() {
            return Some(Event::Editor(EditorEvent::Insert(c)));
        }
        if let Some(&event) = self.keybinds.get(key) {
            return Some(event);
        }
        None
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self {
            keybinds: HashMap::from([
                (
                    Key::new(KeyCode::ESC, KeyMods::NONE),
                    Event::App(AppEvent::Exit),
                ),
                (
                    Key::new(KeyCode::ENTER, KeyMods::NONE),
                    Event::Editor(EditorEvent::NewLine),
                ),
                (
                    Key::new(KeyCode::RIGHT, KeyMods::NONE),
                    Event::Editor(EditorEvent::CursorMove(Move::Right)),
                ),
                (
                    Key::new(KeyCode::LEFT, KeyMods::NONE),
                    Event::Editor(EditorEvent::CursorMove(Move::Left)),
                ),
                (
                    Key::new(KeyCode::UP, KeyMods::NONE),
                    Event::Editor(EditorEvent::CursorMove(Move::Up)),
                ),
                (
                    Key::new(KeyCode::DOWN, KeyMods::NONE),
                    Event::Editor(EditorEvent::CursorMove(Move::Down)),
                ),
                (
                    Key::new(KeyCode::BACKSPACE, KeyMods::NONE),
                    Event::Editor(EditorEvent::Remove),
                ),
                (
                    Key::new(KeyCode::DELETE, KeyMods::NONE),
                    Event::Editor(EditorEvent::Delete),
                ),
                (
                    Key::new(KeyCode::TAB, KeyMods::NONE),
                    Event::Editor(EditorEvent::AddTab),
                ),
                (
                    Key::new(KeyCode::F(1), KeyMods::NONE),
                    Event::Editor(EditorEvent::RemoveTab),
                ),
                (
                    Key::new(KeyCode::CHAR('d'), KeyMods::CTRL),
                    Event::Editor(EditorEvent::DeleteLine),
                ),
                (
                    Key::new(KeyCode::UP, KeyMods::SHIFT | KeyMods::ALT),
                    Event::Editor(EditorEvent::CopyLineAbove),
                ),
                (
                    Key::new(KeyCode::DOWN, KeyMods::SHIFT | KeyMods::ALT),
                    Event::Editor(EditorEvent::CopyLineBelow),
                ),
                (
                    Key::new(KeyCode::HOME, KeyMods::NONE),
                    Event::Editor(EditorEvent::CursorMove(Move::ToFileBegin)),
                ),
                (
                    Key::new(KeyCode::END, KeyMods::NONE),
                    Event::Editor(EditorEvent::CursorMove(Move::ToFileEnd)),
                ),
                (
                    Key::new(KeyCode::LEFT, KeyMods::ALT),
                    Event::Editor(EditorEvent::CursorMove(Move::ToLineBegin)),
                ),
                (
                    Key::new(KeyCode::RIGHT, KeyMods::ALT),
                    Event::Editor(EditorEvent::CursorMove(Move::ToLineEnd)),
                ),
                (
                    Key::new(KeyCode::DOWN, KeyMods::ALT),
                    Event::Editor(EditorEvent::MoveLineDown),
                ),
                (
                    Key::new(KeyCode::UP, KeyMods::ALT),
                    Event::Editor(EditorEvent::MoveLineUp),
                ),
            ]),
        }
    }
}
