use std::rc::Rc;

use crate::App;
use crate::{InputBox, InputMode};

use crate::search_replace::list_files;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use strum::IntoEnumIterator;

pub fn ui<B: Backend>(f: &mut Frame, app: &App) {
    let (left_side, right_side) = layout(f);
    help_message(f, app, &left_side);

    let width = left_side[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = app.input[0].visual_scroll(width as usize); // TODO fix visual scroll later for a per
                                                             // input box version
    input_boxes(f, app, &left_side, scroll);

    set_cursor(f, app, &left_side, scroll);
    side_window(f, app, &right_side);
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
                Constraint::Length(4),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(outer[0]);
    let right_side = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Min(0)])
        .split(outer[1]);
    (left_side, right_side)
}

fn help_message(f: &mut Frame, app: &App, chunks: &Rc<[Rect]>) {
    let (mut text, style) = match app.input_mode {
        InputMode::Normal => (
            Text::from(vec![
                Line::from(vec![
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit"),
                ]),
                Line::from(vec![
                    Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to start editing"),
                ]),
                Line::from(vec![
                    Span::styled("<Tab>", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to switch input boxes"),
                ]),
            ]),
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            Text::from(vec![Line::from(vec![
                Span::styled("<Esc>", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing"),
            ])]),
            Style::default(),
        ),
    };
    text = text.patch_style(style);
    let message = Paragraph::new(text).block(Block::default());
    f.render_widget(message, chunks[0]);
}

fn input_boxes(f: &mut Frame, app: &App, chunks: &Rc<[Rect]>, scroll: usize) {
    let mut add_input_box = |b: InputBox| {
        let input_box = Paragraph::new(app.input[b.pos()].value())
            .style(if b == app.input_box {
                match app.input_mode {
                    InputMode::Editing => Style::default().fg(Color::Yellow),
                    InputMode::Normal => Style::default().fg(Color::LightMagenta),
                }
            } else {
                Style::default()
            })
            .scroll((0, scroll as u16))
            .block(Block::default().borders(Borders::ALL).title(b.title()));

        f.render_widget(input_box, chunks[b.pos() + 1]);
    };
    for input_box in InputBox::iter() {
        add_input_box(input_box);
    }
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
                chunks[1].x
                    + ((app.input[app.input_box.pos()].visual_cursor()).max(scroll) - scroll)
                        as u16
                    + 1,
                // Move one line down, from the border to the input line
                chunks[app.input_box.pos() + 1].y + 1,
            )
        }
    }
}

fn side_window(f: &mut Frame, app: &App, chunks: &Rc<[Rect]>) {
    let mut search_glob: String = String::new();
    app.input[InputBox::Filepath.pos()]
        .value()
        .clone_into(&mut search_glob);
    let res = list_files(search_glob);
    let mut content: Vec<Line> = vec![];
    for line in res.iter() {
        content.push(Line::raw(line))
    }
    f.render_widget(
        Paragraph::new(content).block(Block::default().title("Greeting").borders(Borders::ALL)),
        chunks[0],
    );
}
