use std::rc::Rc;

use crate::models::match_struct::Match;
use crate::App;
use crate::InputBox;

use crate::search::search;

use ratatui::{
    backend::Backend,
    layout::Rect,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_right_side<B: Backend>(f: &mut Frame, app: &App, right_side: &Rc<[Rect]>) {
    side_window(f, app, right_side);
}

fn side_window(f: &mut Frame, app: &App, chunks: &Rc<[Rect]>) {
    let search_pattern = app.input[InputBox::Search.pos()].value().to_string();
    let replacement = app.input[InputBox::Replace.pos()].value().to_string();
    let search_glob = app.input[InputBox::Filepath.pos()].value().to_string();
    let mut res: Vec<Match> = search(search_glob, search_pattern);
    for m in res.iter_mut() {
        m.set_replacement(replacement.clone());
    }
    let content: Vec<Line> = res.iter().flat_map(|line| line.tui_fmt()).collect();
    let block = Block::default().title("Greeting").borders(Borders::ALL);
    let paragraph = Paragraph::new(Text::from(content)).block(block);
    f.render_widget(paragraph, chunks[0]);
}
