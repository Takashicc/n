use std::io;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
    pub fn new() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        let _stdout = io::stdout().into_raw_mode().unwrap();
        for b in io::stdin().keys() {
            match b {
                Ok(b) => match b {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?}\r", c as u8);
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                        }
                    }
                    Key::Ctrl('q') => break,
                    _ => todo!(),
                },
                Err(err) => die(err),
            }
        }
    }
}

fn die(e: io::Error) {
    panic!("{:?}", e);
}
