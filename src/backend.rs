use std::rc::Rc;

use crate::App;
use crate::{InputBox, InputMode};

use crate::search_replace::search;

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
    render_left_side::<B>(f, app, &left_side);
    render_right_side::<B>(f, app, &right_side);
}

fn render_left_side<B: Backend>(f: &mut Frame, app: &App, left_side: &Rc<[Rect]>) {
    help_message(f, app, left_side);
    let width = left_side[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = app.input[0].visual_scroll(width as usize); // TODO fix visual scroll later for a per input box version
    input_boxes(f, app, left_side, scroll);
    set_cursor(f, app, left_side, scroll);
}

fn render_right_side<B: Backend>(f: &mut Frame, app: &App, right_side: &Rc<[Rect]>) {
    side_window(f, app, right_side);
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
    for input_box in InputBox::iter() {
        let input_box_widget = create_input_box_widget(app, &input_box, scroll);
        f.render_widget(input_box_widget, chunks[input_box.pos() + 1]);
    }
}

fn create_input_box_widget<'a, 'b>(app: &'a App, input_box: &'b InputBox, scroll: usize) -> Paragraph<'a> {
    let style = match app.input_mode {
        InputMode::Editing => Style::default().fg(Color::Yellow),
        InputMode::Normal => Style::default().fg(Color::LightMagenta),
    };

    let style = if input_box == &app.input_box_selection {
        style
    } else {
        Style::default()
    };

    Paragraph::new(app.input[input_box.pos().clone()].value())
        .style(style)
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title(input_box.title().clone()))
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
                    + ((app.input[app.input_box_selection.pos()].visual_cursor()).max(scroll) - scroll)
                        as u16
                    + 1,
                // Move one line down, from the border to the input line
                chunks[app.input_box_selection.pos() + 1].y + 1,
            )
        }
    }
}

fn side_window(f: &mut Frame, app: &App, chunks: &Rc<[Rect]>) {
    let search_pattern = app.input[InputBox::Search.pos()].value().to_string();
    let replacement = app.input[InputBox::Replace.pos()].value().to_string();
    let search_glob = app.input[InputBox::Filepath.pos()].value().to_string();
    let mut res: Vec<crate::match_struct::Match> = search(search_glob, search_pattern);
    for m in res.iter_mut() {
        m.set_replacement(replacement.clone());
    }
    let content: Vec<Line> = res.iter().flat_map(|line| line.tui_fmt()).collect();
    let block = Block::default().title("Greeting").borders(Borders::ALL);
    let paragraph = Paragraph::new(Text::from(content)).block(block);
    f.render_widget(paragraph, chunks[0]);
}
