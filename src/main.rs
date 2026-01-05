// ANCHOR: all
use std::{error::Error, io, time::{Duration, Instant}};

use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

mod app;
mod ui;
mod utils;
mod game;

use crate::{
    app::App,
    ui::ui,
};

// ANCHOR: main_all
// ANCHOR: setup_boilerplate
fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    let tick_rate = Duration::from_millis(1);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;
        let delta_time = last_tick.elapsed();
        app.game.update(delta_time);

        last_tick = Instant::now();

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                // If handle_event returns true, it means we should exit
                if app.handle_event(key.code) {
                    return Ok(false);
                }
            }
        }
    }
}
