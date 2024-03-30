use crate::App;

use ratatui::{backend::Backend, Frame};

use super::input_boxes::render_left_side;
use super::layout::layout;
use super::preview_window::render_right_side;

pub fn ui<B: Backend>(f: &mut Frame, app: &App) {
    let (left_side, right_side) = layout(f);
    render_left_side::<B>(f, app, &left_side);
    render_right_side::<B>(f, app, &right_side);
}
