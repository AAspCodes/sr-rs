use tui_input::Input;

use crate::{InputBox, InputMode};

/// App holds the state of the application
pub struct App {
    /// Current value of the input boxes
    pub input: Vec<Input>,
    /// Current input mode
    pub input_mode: InputMode,
    /// Current input box selection
    pub input_box_selection: InputBox,
}

impl Default for App {
    fn default() -> App {
        App {
            input: vec![Input::default(), Input::default(), Input::default()],
            input_mode: InputMode::Normal,
            input_box_selection: InputBox::Search,
        }
    }
}
