use ratatui::{DefaultTerminal, Frame};
mod startbutton;
mod timer;
use startbutton::button;

fn main() -> std::io::Result<()> {
    ratatui::run(button);
    Ok(())
}
