mod args;
mod pipe;

use std::{
    io::{self, Write as _},
    process, time,
};

use crossterm::{QueueableCommand as _, cursor, event, style::Stylize as _, terminal};

use crate::{args::Args, pipe::Pipe};

fn main() {
    let args = Args::new();
    if let Err(err) = run(&args) {
        eprintln!("{} {err:#}", "error:".dark_red().bold());
        process::exit(1);
    }
}

fn run(args: &Args) -> io::Result<()> {
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut rng = rand::rng();
    let (cols, rows) = terminal::size()?;
    let mut colors_iter = args.colors.iter().copied().cycle();
    let mut pipes = (0..args.number)
        .map(|_| {
            let color = colors_iter.next().expect("iterator should be endless");
            Pipe::new(&mut rng, cols, rows, args.straight_prob, color)
        })
        .collect::<Vec<_>>();

    let mut tick = 0u32;
    let tick_duration = time::Duration::from_secs(1) / args.tickrate;

    loop {
        let start = time::Instant::now();

        if tick == args.reset {
            tick = 0;
            stdout.queue(terminal::Clear(terminal::ClearType::All))?;
        }

        let (cols, rows) = terminal::size()?;
        for pipe in &mut pipes {
            pipe.progress(&mut rng, cols, rows);
            pipe.draw(&mut stdout)?;
        }

        stdout.flush()?;

        if event::poll(tick_duration.saturating_sub(start.elapsed()))?
            && let event::Event::Key(_) = event::read()?
        {
            break;
        }

        tick = tick.wrapping_add(1);
    }

    terminal::disable_raw_mode()?;
    crossterm::execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;

    Ok(())
}
