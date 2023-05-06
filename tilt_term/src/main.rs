use tilt_core::App;
use tilt_term::Terminal;

fn main() {
    let terminal = Terminal::new();
    App::new(Box::new(terminal)).run();
}
