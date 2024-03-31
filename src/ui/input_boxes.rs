use std::rc::Rc;

use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use strum::IntoEnumIterator;

use crate::{
    app::App,
    enums::input_enums::{InputBox, InputMode},
};

use super::cursor::set_cursor;

pub fn render_left_side<B: Backend>(f: &mut Frame, app: &App, left_side: &Rc<[Rect]>) {
    help_message(f, app, left_side);
    let width = left_side[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = app.input[0].visual_scroll(width as usize); // TODO fix visual scroll later for a per input box version
    input_boxes(f, app, left_side, scroll);
    set_cursor(f, app, left_side, scroll);
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
                    Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to start editing"),
                ]),
                Line::from(vec![
                    Span::styled(
                        "j",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" to go down"),
                ]),
                Line::from(vec![
                    Span::styled(
                        "k",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" to go up"),
                ]),
            ]),
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Refine => (
            Text::from(vec![
                Line::from(vec![
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit"),
                ]),
                Line::from(vec![
                    Span::styled("<Tab>", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to switch back to refine search"),
                ]),
                Line::from(vec![
                    Span::styled("j", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to go down"),
                ]),
                Line::from(vec![
                    Span::styled("k", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to go up"),
                ]),
                Line::from(vec![
                    Span::styled("d", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to remove a match"),
                ]),
                Line::from(vec![
                    Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to replace a match"),
                ]),
                Line::from(vec![
                    Span::styled("a", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to replace all matches"),
                ]),
            ]),
            Style::default(),
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

fn create_input_box_widget<'a, 'b>(
    app: &'a App,
    input_box: &'b InputBox,
    scroll: usize,
) -> Paragraph<'a> {
    let style = match app.input_mode {
        InputMode::Editing => Style::default().fg(Color::Yellow),
        InputMode::Normal | InputMode::Refine => Style::default().fg(Color::LightMagenta),
    };

    let style = if input_box == &app.input_box_selection {
        style
    } else {
        Style::default()
    };

    Paragraph::new(app.input[input_box.pos().clone()].value())
        .style(style)
        .scroll((0, scroll as u16))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(input_box.title().clone()),
        )
}
