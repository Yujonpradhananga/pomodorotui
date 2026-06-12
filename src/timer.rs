use ratatui::{DefaultTerminal, Frame};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn timer(terminal: &mut DefaultTerminal) {
    let now = Instant::now();
    sleep(Duration::new(0, 0));
    let i: &mut i32 = &mut 0;
    let time: &mut i32 = &mut 1500;
    while i <= time {
        println!("{i}");
        sleep(Duration::new(1, 0));
        terminal.clear();
    }
}
