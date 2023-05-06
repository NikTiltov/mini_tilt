#[derive(Debug, Clone, Copy)]
pub enum Event {
    App(AppEvent),
    Editor(EditorEvent),
}

#[derive(Debug, Clone, Copy)]
pub enum AppEvent {
    Exit,
}

#[derive(Debug, Clone, Copy)]
pub enum EditorEvent {
    Insert(char),
    NewLine,
    CursorMove(Move),
    Remove,
    Delete,
    AddTab,
    RemoveTab,
    Copy,
    Paste,
    Cut,
    CopyLineAbove,
    CopyLineBelow,
    MoveLineUp,
    MoveLineDown,
    DeleteLine,
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Left,
    Right,
    Down,
    Up,
    ToFileBegin,
    ToFileEnd,
    ToLineBegin,
    ToLineEnd,
}
