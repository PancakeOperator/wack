
mod actions;
use actions::{
   editor::*,
};


use crossterm::{ //event::Event::*,
    terminal, Result,
};


fn main() -> Result<()> {
    let mut editor = Editor::new()?;

    editor.start()?;
    Ok(())
}

