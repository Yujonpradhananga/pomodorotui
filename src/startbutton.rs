use crate::timer::{self, timer};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    macros::ratatui_core::{terminal, widgets},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

pub fn button(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| {
            frame.render_widget("press any button to start timer", frame.area());
        })?;
        if crossterm::event::read()?.is_key_press() {
            terminal.clear();
            timer::timer(terminal);
        }
    }
}
