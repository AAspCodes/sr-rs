use strum::EnumIter;

pub enum InputMode {
    Normal,
    Editing,
    Refine,
}

/// Represents the different types of input boxes.
#[derive(PartialEq, EnumIter)]
pub enum InputBox {
    Search,
    Replace,
    Filepath,
}

impl InputBox {
    /// Returns the next input box in the sequence.
    pub fn next(&self) -> Self {
        match self {
            Self::Search => Self::Replace,
            Self::Replace => Self::Filepath,
            Self::Filepath => Self::Search,
        }
    }

    /// Returns the previous input box in the sequence.
    pub fn prev(&self) -> Self {
        match self {
            Self::Search => Self::Filepath,
            Self::Replace => Self::Search,
            Self::Filepath => Self::Replace,
        }
    }

    /// Returns the position of the input box.
    pub fn pos(&self) -> usize {
        match self {
            Self::Search => 0,
            Self::Replace => 1,
            Self::Filepath => 2,
        }
    }

    /// Returns the title of the input box.
    pub fn title(&self) -> String {
        match self {
            Self::Search => "Search".into(),
            Self::Replace => "Replace".into(),
            Self::Filepath => "FilePath".into(),
        }
    }
}
