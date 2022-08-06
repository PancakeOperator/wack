
mod help;

mod actions;
use actions::{editor_keys::read_spook, 
    screen_commands::{enter_alt_screen, clear_screen_now}
};

//use help::error::die;

use crossterm::{ //event::Event::*,
    terminal, Result,
};


fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    loop {
        enter_alt_screen()?;
        clear_screen_now()?;
        if read_spook() {
            break;
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}

