//use std::io::{self, stdout};
//
//use crossterm::{
//    event::{self, Event, KeyCode},
//    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
//    ExecutableCommand,
//};
//use ratatui::{prelude::*, widgets::*};
//
//fn main() -> io::Result<()> {
//    enable_raw_mode()?;
//    stdout().execute(EnterAlternateScreen)?;
//    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
//
//    let mut should_quit = false;
//    while !should_quit {
//        terminal.draw(ui)?;
//        should_quit = handle_events()?;
//    }
//
//    disable_raw_mode()?;
//    stdout().execute(LeaveAlternateScreen)?;
//    Ok(())
//}
//
//fn handle_events() -> io::Result<bool> {
//    if event::poll(std::time::Duration::from_millis(50))? {
//        if let Event::Key(key) = event::read()? {
//            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
//                return Ok(true);
//            }
//        }
//    }
//    Ok(false)
//}
//
//fn ui(frame: &mut Frame) {
//    let main_layout = Layout::new(
//        Direction::Horizontal,
//        [
//            Constraint::Percentage(30),
//            Constraint::Percentage(70),
//        ],
//    )
//    .split(frame.size());
//    frame.render_widget(
//        Paragraph::new("todo but text boxes in here")
//            .block(Block::default().title("search and replace box").borders(Borders::ALL)),
//        main_layout[0],
//    );
//    frame.render_widget(
//        Paragraph::new("i'm a file")
//            .block(Block::default().title("see file changes box").borders(Borders::ALL)),
//        main_layout[1],
//    );
//}

/// This example is taken from https://raw.githubusercontent.com/fdehau/tui-rs/master/examples/user_input.rs
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
                    KeyCode::Tab => app.box_num = (app.box_num + 1) % 2,
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        if app.box_num == 0 {
                            app.messages1.push(app.input.value().into());
                        } else {
                            app.messages2.push(app.input.value().into());
                        }
                        app.input.reset();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {
                        app.input.handle_event(&Event::Key(key));
                    }
                },
            }
        }
    }
}

//fn ui<B: Backend>(f: &mut Frame, app: &App) {
//    let chunks = Layout::default()
//        .direction(Direction::Vertical)
//        .margin(2)
//        .constraints(
//            [
//                Constraint::Length(1),
//                Constraint::Length(3),
//                Constraint::Length(3),
//                Constraint::Min(1),
//                Constraint::Min(1),
//            ]
//            .as_ref(),
//        )
//        .split(f.size());
//
//    let (msg, style) = match app.input_mode {
//        InputMode::Normal => (
//            vec![
//                Span::raw("Press "),
//                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
//                Span::raw(" to exit, "),
//                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
//                Span::raw(" to start editing."),
//            ],
//            Style::default().add_modifier(Modifier::RAPID_BLINK),
//        ),
//        InputMode::Editing => (
//            vec![
//                Span::raw("Press "),
//                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
//                Span::raw(" to stop editing, "),
//                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
//                Span::raw(" to record the message"),
//            ],
//            Style::default(),
//        ),
//    };
//    let mut text = Text::from(Line::from(msg));
//    text = text.patch_style(style);
//    let help_message = Paragraph::new(text);
//    f.render_widget(help_message, chunks[0]);
//
//    let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
//
//    let scroll = app.input.visual_scroll(width as usize);
//    let search_input = Paragraph::new(if app.box_num == 0 {app.input.value()} else {""})
//        .style(match app.box_num {
//            0 => match app.input_mode {
//                InputMode::Editing => Style::default().fg(Color::Yellow),
//                InputMode::Normal => Style::default().fg(Color::LightMagenta),
//                }
//            _ => Style::default(),
//        })
//        .scroll((0, scroll as u16))
//        .block(Block::default().borders(Borders::ALL).title("Search"));
//    let replace_input = Paragraph::new(if app.box_num == 1 {app.input.value()} else {""})
//        .style(match app.box_num {
//            1 => match app.input_mode {
//                InputMode::Editing => Style::default().fg(Color::Yellow),
//                InputMode::Normal => Style::default().fg(Color::LightMagenta),
//                }
//            _ => Style::default(),
//        })
//        .scroll((0, scroll as u16))
//        .block(Block::default().borders(Borders::ALL).title("Replace"));
//    f.render_widget(search_input, chunks[1]);
//    f.render_widget(replace_input, chunks[2]);
//    match app.input_mode {
//        InputMode::Normal =>
//            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
//            {}
//
//        InputMode::Editing => {
//            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
//            f.set_cursor(
//                // Put cursor past the end of the input text
//                chunks[1].x
//                    + ((app.input.visual_cursor()).max(scroll) - scroll) as u16
//                    + 1,
//                // Move one line down, from the border to the input line
//                chunks[app.box_num as usize + 1].y + 1,
//            )
//        }
//    }
//
//    let messages1: Vec<ListItem> = app
//        .messages1
//        .iter()
//        .enumerate()
//        .map(|(i, m)| {
//            let content = vec![Line::from(Span::raw(format!("{}: {}", i, m)))];
//            ListItem::new(content)
//        })
//        .collect();
//    let messages2: Vec<ListItem> = app
//        .messages2
//        .iter()
//        .enumerate()
//        .map(|(i, m)| {
//            let content = vec![Line::from(Span::raw(format!("{}: {}", i, m)))];
//            ListItem::new(content)
//        })
//        .collect();
//    let messages1 = List::new(messages1)
//        .block(Block::default().borders(Borders::ALL).title("Messages1"));
//    let messages2 = List::new(messages2)
//        .block(Block::default().borders(Borders::ALL).title("Messages2"));
//    f.render_widget(messages1, chunks[3]);
//    f.render_widget(messages2, chunks[4]);
//}
