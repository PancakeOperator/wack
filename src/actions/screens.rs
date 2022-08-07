//std crate
use std::io::{stdout, Write, Stdout};

//extra crate
use crossterm::{terminal ,QueueableCommand, Result, style::Print, cursor};
use errno::errno;



pub struct Screen {
    stdout: Stdout,
    height: u16,
    width: u16
}

impl Screen {
    pub fn new_screen() -> Result<Self> {
        let (columns, rows) = crossterm::terminal::size()?;
        Ok(Self {
            width: columns,
            height: rows,
            stdout: stdout(),
        })  
    }
    pub fn draw_row(&mut self) -> Result<()> {
        for row in 1..self.height {
            self.stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(Print("~".to_string()))?;
        }
        Ok(())
    }
}
