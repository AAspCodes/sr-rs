use std::rc::Rc;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::App;
use crate::InputMode;

pub fn ui<B: Backend>(f: &mut Frame, app: &App) {
    let (left_side, right_side) = layout(f);
    help_message(f, app, &left_side);

    let width = left_side[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = app.input.visual_scroll(width as usize);
    input_boxes(f, app, &left_side, scroll);

    set_cursor(f, app, &left_side, scroll);
    side_window(f, &right_side);
}

fn layout(f: &mut Frame) -> (Rc<[Rect]>, Rc<[Rect]>) {
    let outer = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.size());

    let left_side = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(outer[0]);
    let right_side = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([
                     Constraint::Min(0),
        ])
        .split(outer[1]);
    (left_side, right_side)
}

fn help_message(f: &mut Frame, app: &App, chunks: &Rc<[Rect]>) {
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Line::from(msg));
    text = text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);
}

fn input_boxes(f: &mut Frame, app: &App, chunks: &Rc<[Rect]>, scroll: usize) {
    let search_input = Paragraph::new(if app.box_num == 0 {
        app.input.value()
    } else {
        ""
    })
    .style(match app.box_num {
        0 => match app.input_mode {
            InputMode::Editing => Style::default().fg(Color::Yellow),
            InputMode::Normal => Style::default().fg(Color::LightMagenta),
        },
        _ => Style::default(),
    })
    .scroll((0, scroll as u16))
    .block(Block::default().borders(Borders::ALL).title("Search"));
    let replace_input = Paragraph::new(if app.box_num == 1 {
        app.input.value()
    } else {
        ""
    })
    .style(match app.box_num {
        1 => match app.input_mode {
            InputMode::Editing => Style::default().fg(Color::Yellow),
            InputMode::Normal => Style::default().fg(Color::LightMagenta),
        },
        _ => Style::default(),
    })
    .scroll((0, scroll as u16))
    .block(Block::default().borders(Borders::ALL).title("Replace"));
    f.render_widget(search_input, chunks[1]);
    f.render_widget(replace_input, chunks[2]);
}

fn set_cursor(f: &mut Frame, app: &App, chunks: &Rc<[Rect]>, scroll: usize) {
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[app.box_num as usize + 1].y + 1,
            )
        }
    }
}

fn side_window(f: &mut Frame, chunks: &Rc<[Rect]>) {
    f.render_widget(
        Paragraph::new("Hello World!")
            .block(Block::default().title("Greeting").borders(Borders::ALL)),
            chunks[0],
            );
}
