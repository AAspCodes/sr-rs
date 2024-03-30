use strum::EnumIter;

pub enum InputMode {
    Normal,
    Editing,
}

#[derive(PartialEq, EnumIter)]
pub enum InputBox {
    Search,
    Replace,
    Filepath,
}

impl InputBox {
    pub fn next(self: &InputBox) -> InputBox {
        match self {
            InputBox::Search => InputBox::Replace,
            InputBox::Replace => InputBox::Filepath,
            InputBox::Filepath => InputBox::Search,
        }
    }

    pub fn pos(self: &InputBox) -> usize {
        match self {
            InputBox::Search => 0 as usize,
            InputBox::Replace => 1 as usize,
            InputBox::Filepath => 2 as usize,
        }
    }

    pub fn title(self: &InputBox) -> String {
        match self {
            InputBox::Search => "Search".into(),
            InputBox::Replace => "Replace".into(),
            InputBox::Filepath => "FilePath".into(),
        }
    }
}
