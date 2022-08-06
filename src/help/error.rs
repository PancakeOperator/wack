use errno::errno;
use crate::actions::screen_commands::clear_screen_now;
use std::io::stdout;
use crossterm::terminal::*;
pub fn die<S: Into<String>>(message: S) {
    let mut stdout = stdout();
    let _ = clear_screen_now(&mut stdout);
    let _ = disable_raw_mode();
    eprintln!("{} {}", message.into(), errno());
    std::process::exit(1);
}

