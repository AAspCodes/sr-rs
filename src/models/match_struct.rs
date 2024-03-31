use std::fmt;

use ratatui::{
    style::Style,
    text::{Line, Span},
};

#[derive(Debug, Clone)]
pub struct Match {
    filepath: String,
    file_index_start: usize,
    match_length: usize,
    start_on_line: usize,
    end_on_line: usize,
    replacement: String,
    line: String,
    line_num: usize,
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Filepath: {}, Start: {}, End: {}, Line: {}",
            self.filepath, self.start_on_line, self.end_on_line, self.line
        )
    }
}

impl Match {
    pub fn new(
        filepath: String,
        file_index_start: usize,
        match_length: usize,
        start_on_line: usize,
        end_on_line: usize,
        replacement: String,
        line: String,
        line_num: usize,
    ) -> Self {
        Self {
            filepath,
            file_index_start,
            match_length,
            start_on_line,
            end_on_line,
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
            .nth(self.start_on_line)
            .unwrap_or((0, ' '))
            .0;
        let end_byte_index = self
            .line
            .char_indices()
            .nth(self.end_on_line)
            .unwrap_or((self.line.len(), ' '))
            .0;

        (start_byte_index, end_byte_index)
    }

    pub fn set_replacement(&mut self, replacement: String) {
        self.replacement = replacement;
    }

    pub fn get_filepath(&self) -> &str {
        &self.filepath
    }
    pub fn get_replacement(&self) -> &str {
        &self.replacement
    }
    pub fn get_file_index_start(&self) -> usize {
        self.file_index_start
    }
    pub fn get_match_length(&self) -> usize {
        self.match_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_byte_indices_with_multibyte_chars() {
        let m = Match {
            filepath: String::from("test.rs"),
            file_index_start: 0,
            match_length: 1,
            line: String::from("Hello, 😀 world!"),
            line_num: 0,
            start_on_line: 7,
            end_on_line: 8,
            replacement: String::from(""),
        };

        let (byte_start, byte_end) = m.get_byte_indices();
        // The start index should be 7
        assert_eq!(byte_start, 7);
        // the end index should be 11 not 8, because "😀" takes 4 bytes.
        assert_eq!(byte_end, 11);
    }
}
