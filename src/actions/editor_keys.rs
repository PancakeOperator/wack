use crate::actions::read_keys::*;
use crossterm::event::{KeyEvent, KeyModifiers, KeyCode};

pub fn read_spook() -> bool {
    let k = read_key_event();
    match k {
        Ok(KeyEvent{ 
            code: KeyCode::Char('q'), 
            modifiers: KeyModifiers::CONTROL
        }) => return true,
        _ => {},
    }
    false
}