use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self { quit: false }
    }

    pub fn run(&mut self) {
        let _stdout = io::stdout().into_raw_mode().unwrap();

        loop {
            if let Err(err) = self.clear_screen() {
                die(&err);
            }

            if self.quit {
                break;
            }

            if let Err(err) = self.process_keypress() {
                die(&err);
            }
        }
    }

    fn clear_screen(&self) -> Result<(), io::Error> {
        print!("{}", termion::clear::All);
        io::stdout().flush()
    }

    fn process_keypress(&mut self) -> Result<(), io::Error> {
        match read_key()? {
            Key::Ctrl('q') => self.quit = true,
            _ => (),
        }

        Ok(())
    }
}

fn read_key() -> Result<Key, io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: &io::Error) {
    panic!("{:?}", e);
}
