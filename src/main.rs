mod help;
mod actions;
use actions::{read_keys, editor_keys::read_spook};

use help::error::die;

use crossterm::{ event::Event::*,
    terminal, Result,
};


fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    loop {
        if read_spook() {
            break;
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}

