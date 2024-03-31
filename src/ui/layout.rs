use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn layout(f: &mut Frame) -> (Rc<[Rect]>, Rc<[Rect]>) {
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
                Constraint::Length(7),
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
