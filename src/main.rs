
mod help;

mod actions;
use actions::{editor_keys::read_spook, 
    screen_commands::{enter_alt_screen, }
};

use help::error::die;

use crossterm::{ //event::Event::*,
    terminal, Result,
};


fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    loop {
        if enter_alt_screen().is_err() {
            die("was not able to enter alternate screen");
        }
        /*if clear_screen_now().is_err() {
            die("unable to clear screen");
            //might remove later
        }*/ 
        // i don't think i need clear screen when i can just go into another screen
        if read_spook() {
            break;
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}

