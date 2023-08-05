mod app;
mod event;
mod request;
mod ui;

use app::{App, InputMode};
use crossterm::{
    event::{self as crossterm_event, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Error, Stdout};

fn main() -> Result<(), Error> {
    let mut terminal = setup_terminal()?;

    let mut app = App::default();

    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        restore_terminal().unwrap();
        original_hook(panic);
    }));

    let res = run(&mut terminal, &mut app);

    restore_terminal()?;

    if let Err(err) = res {
        println!("{}", err);
    }

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Error> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal() -> Result<(), Error> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;

        if let Event::Key(key) = crossterm_event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => match app.popup {
                        Some(_) => {
                            app.popup = None;
                        }
                        None => {
                            return Ok(());
                        }
                    },
                    _ => {}
                },
                _ => {}
            }

            event::handle_input(app, key);
        }
    }
}
