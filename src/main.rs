use ratatui::{DefaultTerminal, Frame};
mod timer;
use timer::timer;

fn main() -> std::io::Result<()> {
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    timer();
    Ok(())
}
