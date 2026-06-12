use crossterm::{event, terminal};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::{DefaultTerminal, Frame};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn timer(terminal: &mut DefaultTerminal) {
    let now = Instant::now();
    sleep(Duration::new(2, 0));
    println!("{}", now.elapsed().as_secs());
}
