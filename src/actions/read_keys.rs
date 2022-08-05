use crossterm::event::Event::Key;
use crossterm::Result;
use crossterm::event::{read, KeyEvent};

use crate::help::error::die;
pub fn read_key_event() -> Result<KeyEvent> {
    loop {
        if let Ok(event) = read() {
            if let Key(key_event) = event {
                return Ok(key_event);
            }
        } else {
            die("read");
            break;
        }
    }
    unreachable!();
}