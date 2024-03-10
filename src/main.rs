use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io};
use tui_input::backend::crossterm::EventHandler;
mod backend;
use backend::ui;
mod app;
use app::App;

enum InputMode {
    Normal,
    Editing,
}

#[derive(PartialEq)]
enum InputBox {
    Search,
    Replace,
    Filepath,
}

impl InputBox {
    fn next(self: InputBox) -> InputBox {
        match self {
            InputBox::Search => InputBox::Replace,
            InputBox::Replace => InputBox::Filepath,
            InputBox::Filepath => InputBox::Search,
        }
    }

    fn pos(self: &InputBox) -> usize {
        match self {
            InputBox::Search => 0 as usize,
            InputBox::Replace => 1 as usize,
            InputBox::Filepath => 2 as usize,
        }
    }

    fn title(self: &InputBox) -> String {
        match self {
            InputBox::Search => "Search".into(),
            InputBox::Replace => "Replace".into(),
            InputBox::Filepath => "FilePath".into(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
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
