use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, layout::Alignment, widgets::Paragraph};
use std::io;
use std::time::{Duration, Instant};

pub fn button(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let total_seconds: i32 = 1500; // 25 minutes
    let mut remaining = total_seconds;
    let tick_rate = Duration::from_secs(1);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| {
            let minutes = remaining / 60;
            let seconds = remaining % 60;
            let text = format!("{:02}:{:02}", minutes, seconds);
            let paragraph = Paragraph::new(text).alignment(Alignment::Center);
            frame.render_widget(paragraph, frame.area());
        })?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && key.code == KeyCode::Char('q')
        {
            return Ok(());
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
            if remaining > 0 {
                remaining -= 1;
            } else {
                break;
            }
        }
    }

    Ok(())
}
