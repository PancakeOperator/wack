//std crate
use std::io::{stdout, Write, Stdout};

//extra crate
use crossterm::{terminal::{Clear, ClearType, EnterAlternateScreen},QueueableCommand, Result, style::Print};
use crossterm::cursor::{MoveTo};
use crossterm::cursor;


pub fn clear_screen_now(stdout: &mut Stdout) -> Result<()> {
    stdout
        .queue(Clear(ClearType::All))?
        .queue(cursor::MoveTo(0,0))?
        .flush()
}

pub fn draw_row(stdout: &mut Stdout) -> Result<()> {
    for row in 1..24 {
        stdout
            .queue(cursor::MoveTo(0, row))?
            .queue(Print("~".to_string()))?;
    }
    Ok(())
}

pub fn enter_alt_screen() -> Result<()> {
    let mut stdout = stdout();
    
    clear_screen_now(&mut stdout)?;
    draw_row(&mut stdout)?;
    stdout.queue(cursor::MoveTo(0,0))?.flush()

    
}