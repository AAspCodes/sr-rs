use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Terminal};

use std::{error::Error, io};
use tui_input::backend::crossterm::EventHandler;
mod ui;
use ui::ui as ui_func;
// todo rename ui_func

mod app;
use app::App;

mod tui;
use tui::{restore_terminal, setup_terminal};
mod enums;
use enums::input_enums::{InputBox, InputMode};

mod logging;
use logging::init_logger;
mod models;
mod search;

fn main() -> Result<(), Box<dyn Error>> {
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
        terminal.draw(|f| ui_func::<B>(f, &app))?;

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
                        app.input_box_selection = app.input_box_selection.next();
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
                        app.input[app.input_box_selection.pos()].handle_event(&Event::Key(key));
                    }
                },
            }
        }
    }
}
