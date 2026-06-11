use ratatui::{DefaultTerminal, Frame};
mod timer;
use timer::timer;

fn main() -> std::io::Result<()> {
    ratatui::run(timer)?;
    Ok(())
}
