use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Terminal};
use search::replace;
use std::{error::Error, io};
use tui_input::backend::crossterm::EventHandler;

use app::App;
use enums::input_enums::{InputBox, InputMode};
use logging::init_logger;
use tui::{restore_terminal, setup_terminal};
use ui::ui as user_interface;

mod app;
mod enums;
mod logging;
mod models;
mod search;
mod tui;
mod ui;

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

/// Main function that starts the application
fn run() -> Result<(), Box<dyn Error>> {
    init_logger()?;
    let sr_logging_art = "
######################
    ######  ######
   #      # #     #
   #        #     #
    ######  ######
          # #   # 
   #      # #    # 
    ######  #     #
######################
 ";
    log::info!("sr is starting up");
    log::info!("{}", sr_logging_art);

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
        terminal.draw(|f| user_interface::<B>(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('i') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Tab => {
                        app.input_mode = InputMode::Refine;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('j') => {
                        app.input_box_selection = app.input_box_selection.next();
                    }
                    KeyCode::Char('k') => {
                        app.input_box_selection = app.input_box_selection.prev();
                    }
                    KeyCode::Char('r') => match replace(&app) {
                        Err(e) => {
                            log::error!("Failed to replace: {}", e);
                        }
                        Ok(_) => {
                            log::info!("Successfully replaced matches");
                        }
                    },
                    _ => {}
                },
                InputMode::Refine => match key.code {
                    KeyCode::Tab => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('j') => {
                        log::info!("go to next match");
                    }
                    KeyCode::Char('k') => {
                        log::info!("go to previous match");
                    }
                    KeyCode::Char('d') => {
                        log::info!("delete match");
                    }
                    KeyCode::Char('r') => {
                        log::info!("replace match");
                    }
                    KeyCode::Char('a') => {
                        log::info!("replace all matches");
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {
                        app.input[app.input_box_selection.pos()].handle_event(&Event::Key(key));
                    }
                },
            }
        }
    }
}
