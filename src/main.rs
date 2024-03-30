use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Terminal};

use std::{error::Error, io};
use tui_input::backend::crossterm::EventHandler;
mod backend;
use backend::ui;
mod app;
use app::App;

mod tui;
use tui::{restore_terminal, setup_terminal};
mod input_enums;
use input_enums::{InputBox, InputMode};

mod logging;
use logging::init_logger;
mod match_struct;
mod search_replace;

fn main() -> Result<(), Box<dyn Error>> {
    init_logger()?;
    log::info!("sr is starting up");
    // setup terminal
    let mut terminal = setup_terminal()?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    restore_terminal()?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::<B>(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Tab => {
                        app.input_box = app.input_box.next();
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        // TODO send input contents to right side, as temp, later trigger search
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {
                        app.input[app.input_box.pos()].handle_event(&Event::Key(key));
                    }
                },
            }
        }
    }
}
