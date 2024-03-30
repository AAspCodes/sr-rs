use std::fmt;

use ratatui::{
    style::Style,
    text::{Line, Span},
};

#[derive(Debug)]
pub struct Match {
    filepath: String,
    start: usize,
    end: usize,
    replacement: String,
    line: String,
    line_num: usize,
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Filepath: {}, Start: {}, End: {}, Line: {}",
            self.filepath, self.start, self.end, self.line
        )
    }
}

impl Match {
    pub fn new(
        filepath: String,
        start: usize,
        end: usize,
        replacement: String,
        line: String,
        line_num: usize,
    ) -> Self {
        Self {
            filepath,
            start,
            end,
            replacement,
            line,
            line_num,
        }
    }
    pub fn tui_fmt(&self) -> Vec<Line> {
        let (start_byte_index, end_byte_index) = self.get_byte_indices();

        let spans = vec![
            Span::raw(format!("line: {} \t", self.line_num)),
            Span::raw(&self.line[..start_byte_index]),
            Span::styled(
                &self.line[start_byte_index..end_byte_index],
                Style::default().fg(ratatui::style::Color::Red),
            ),
            Span::styled(
                &self.replacement,
                Style::default().fg(ratatui::style::Color::Green),
            ),
            Span::raw(&self.line[end_byte_index..]),
        ];

        vec![Span::raw(&self.filepath).into(), spans.into()]
    }

    fn get_byte_indices(&self) -> (usize, usize) {
        let start_byte_index = self
            .line
            .char_indices()
            .nth(self.start)
            .unwrap_or((0, ' '))
            .0;
        let end_byte_index = self
            .line
            .char_indices()
            .nth(self.end)
            .unwrap_or((self.line.len(), ' '))
            .0;

        (start_byte_index, end_byte_index)
    }

    pub fn set_replacement(&mut self, replacement: String) {
        self.replacement = replacement;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_byte_indices_with_multibyte_chars() {
        let m = Match {
            filepath: String::from("test.rs"),
            line: String::from("Hello, 😀 world!"),
            line_num: 1,
            start: 7,
            end: 8,
            replacement: String::from(""),
        };

        let (byte_start, byte_end) = m.get_byte_indices();
        // The start index should be 13, not 7, because "😀" takes 4 bytes.
        assert_eq!(byte_start, 13);
        // The end index should also be 13, because the end index is exclusive.
        assert_eq!(byte_end, 13);
    }

}