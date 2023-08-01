mod app;
mod input_handling;
mod request;
mod ui;

use app::{App, InputMode};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Error, Stdout};
use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};

fn main() -> Result<(), Error> {
    let mut terminal = setup_terminal()?;

    SyntaxSet::load_defaults_newlines();
    ThemeSet::load_defaults();

    let mut app = App::default();

    run(&mut terminal, &mut app)?;

    restore_terminal(&mut terminal)?;

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Error> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> Result<(), Error> {
    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if app.input_mode == InputMode::Normal && key.code == KeyCode::Char('q') {
                return Ok(());
            }

            input_handling::handle_input(app, key);
        }
    }
}
