use std::io;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod editor;
use editor::Editor;

fn main() {
    Editor::new().run();
}
