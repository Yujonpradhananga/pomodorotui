use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

fn quit() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
        },
    }
    Ok(false)
}
