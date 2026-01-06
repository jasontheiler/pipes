use crossterm::{cursor, style::Stylize as _, terminal};

fn main() {
    if let Err(err) = run() {
        eprintln!("{} {err:#}", "error:".dark_red().bold());
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();

    terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    crossterm::event::read()?;

    terminal::disable_raw_mode()?;
    crossterm::execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;

    Ok(())
}
