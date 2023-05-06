use crate::action::Action;
use crate::editor::Editor;
use crate::event::AppEvent;
use crate::event_handler::EventHandler;
use crate::{event::Event, Interface};

pub struct App {
    interface: Box<dyn Interface>,
    editors: Vec<Editor>,
    editor_idx: Option<usize>,
    event_handler: EventHandler,
    running: bool,
}

impl App {
    pub fn new(interface: Box<dyn Interface>) -> Self {
        Self {
            interface,
            editors: Vec::new(),
            editor_idx: None,
            event_handler: Default::default(),
            running: false,
        }
    }

    pub fn run(&mut self) {
        self.start();
        self.render();
        while self.running {
            self.update();
            self.render();
        }
        self.close();
    }

    fn start(&mut self) {
        self.editors.push(Editor::new(None));
        self.editor_idx = Some(0);
        self.running = true;
    }

    fn update(&mut self) {
        if let Action::Key(key) = self.interface.get_action() {
            if let Some(event) = self.event_handler.handle(&key) {
                self.execute(event);
            }
        }
    }

    fn render(&mut self) {
        if let Some(idx) = self.editor_idx {
            if let Some(editor) = self.editors.get(idx) {
                let cursor = editor.cursor();
                self.interface
                    .set_message(format!("{} {}", cursor.row, cursor.cln));
                self.interface.render(editor.buffer(), editor.cursor());
            }
        }
    }

    fn close(&mut self) {}

    fn current_editor(&mut self) -> Option<&mut Editor> {
        if let Some(idx) = self.editor_idx {
            return self.editors.get_mut(idx);
        }
        None
    }

    fn execute(&mut self, event: Event) {
        match event {
            Event::App(event) => match event {
                AppEvent::Exit => self.running = false,
            },
            Event::Editor(event) => {
                if let Some(editor) = self.current_editor() {
                    editor.execute(event);
                }
            }
        }
    }
}
