//std crate
use std::io::{stdout, Write, Stdout};

//extra crate
use crossterm::event::{KeyEvent, read, KeyModifiers, KeyCode};
use crossterm::{terminal ,QueueableCommand, Result, cursor, terminal::EnterAlternateScreen};
use errno::errno;
use crossterm::event::Event::Key;

use crate::actions::screens::Screen;

pub struct Editor {
    screen: Screen,
}

impl Editor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            screen: Screen::new_screen()?,
        })
    }
    pub fn start(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        loop {
            if self.enter_alt_screen().is_err() {
                self.die("was not able to enter alternate screen");
            }
            if self.read_spook() {
                break;
            }
        }
        terminal::disable_raw_mode()?;
        Ok(())
    }

   

    pub fn enter_alt_screen(&mut self) -> Result<()> {
        let mut stdout = stdout();
        self.screen.clear(&mut stdout)?;
        self.screen.draw_row()?;
        stdout.queue(cursor::MoveTo(0,0))?.flush()
        
    }

    pub fn die<S: Into<String>>(&mut self, message: S) {
        let mut stdout = stdout();
        let _ = self.screen.clear(&mut stdout);
        let _ = terminal::disable_raw_mode();
        eprintln!("{} {}", message.into(), errno());
        std::process::exit(1);
    }

    pub fn read_key_event(&mut self) -> Result<KeyEvent> {
        loop {
            if let Ok(event) = read() {
                if let Key(key_event) = event {
                    return Ok(key_event);
                }
            } else {
                self.die("read");
                break;
            }
        }
        unreachable!();
    }

    pub fn read_spook(&mut self) -> bool {
        let k = self.read_key_event();
        match k {
            Ok(KeyEvent{ 
                code: KeyCode::Char('q'), 
                modifiers: KeyModifiers::CONTROL
            }) => return true,
            _ => {},
        }
        false
    }
 
}





