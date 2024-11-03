mod app;
mod cli;
mod event;
mod request;
mod ui;

use app::{App, InputMode};
use clap::Parser;
use crossterm::{
    event::{self as crossterm_event, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::{self, Error, Stdout},
    time::Duration,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = cli::Cli::parse();

    let mut terminal = setup_terminal()?;

    let mut app = App::default();

    if let Some(url) = cli.url {
        app.endpoint.text = url;
    }

    if let Some(method) = cli.method {
        app.method = method;
    }

    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        restore_terminal().unwrap();
        original_hook(panic);
    }));

    run(&mut terminal, &mut app).await?;

    restore_terminal()?;

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

async fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;

        if let Ok(res) = app.res_rx.try_recv() {
            app.response = res;
            app.is_loading = false;
        }

        if crossterm_event::poll(Duration::from_millis(250))? {
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

                event::handle_input(app, key).await;
            }
        }
    }
}
