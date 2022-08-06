
use crossterm::{terminal::{Clear, ClearType, EnterAlternateScreen},QueueableCommand, Result};
use std::io::{stdout, Write};


pub fn clear_screen_now() -> Result<()> {
    let mut stdout = stdout();

    stdout
        .queue(Clear(ClearType::All))?;
    stdout.flush()?;

    Ok(())
}

pub fn enter_alt_screen() -> Result<()> {
    let mut stdout = stdout();

    stdout
        .queue(EnterAlternateScreen)?;
    stdout.flush()?;

    Ok(())
}