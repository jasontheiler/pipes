use crossterm::{cursor, terminal};

fn main() -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();

    terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    crossterm::event::read()?;

    terminal::disable_raw_mode()?;
    crossterm::execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;

    Ok(())
}
