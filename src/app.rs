use tui_input::Input;

use crate::InputMode;

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    pub input: Input,
    /// Current input mode
    pub input_mode: InputMode,
    pub box_num: u8,
    /// History of recorded messages
    pub messages1: Vec<String>,
    pub messages2: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: Input::default(),
            input_mode: InputMode::Normal,
            box_num: 0,
            messages1: Vec::new(),
            messages2: Vec::new(),
        }
    }
}
