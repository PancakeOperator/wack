use errno::errno;
use crossterm::terminal::*;
pub fn die<S: Into<String>>(message: S) {
    let _ = disable_raw_mode();
    eprintln!("{} {}", message.into(), errno());
    std::process::exit(1)
}

