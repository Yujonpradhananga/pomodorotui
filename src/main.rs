mod start_button;
mod timer;
use ratatui::run;
use start_button::button;

fn main() -> std::io::Result<()> {
    let _run = run(button);
    Ok(())
}
