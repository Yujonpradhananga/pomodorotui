use crate::timer::{self};
use color_eyre::eyre::Result;
use ratatui::DefaultTerminal;

pub fn button(terminal: &mut DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            frame.render_widget("press any button to start timer", frame.area());
        })?;
        if crossterm::event::read()?.is_key_press() {
            terminal.clear()?;
            timer::timer(terminal);
        }
    }
}
