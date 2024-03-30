use std::rc::Rc;

use crate::App;
use crate::InputMode;

use ratatui::{layout::Rect, Frame};

pub fn set_cursor(f: &mut Frame, app: &App, chunks: &Rc<[Rect]>, scroll: usize) {
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x
                    + ((app.input[app.input_box_selection.pos()].visual_cursor()).max(scroll)
                        - scroll) as u16
                    + 1,
                // Move one line down, from the border to the input line
                chunks[app.input_box_selection.pos() + 1].y + 1,
            )
        }
    }
}
