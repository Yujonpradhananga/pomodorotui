use crossterm::event;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::{DefaultTerminal, Frame};

pub fn timer(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| {
            frame.render_widget("niga", frame.area());
        })?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}
