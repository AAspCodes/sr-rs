use core::fmt;
use glob::glob;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};
use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct Match {
    filepath: String,
    start: usize,
    end: usize,
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
    pub fn tui_fmt(&self) -> Vec<Line> {
        let (start_byte_index, end_byte_index) = self.get_byte_indices();

        let spans = vec![
            Span::raw(format!("line: {} \t", self.line_num)),
            Span::raw(&self.line[..start_byte_index]),
            Span::styled(
                &self.line[start_byte_index..end_byte_index],
                Style::default().add_modifier(Modifier::BOLD),
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
}

pub fn search(path_g: String, search_pattern: String) -> Vec<Match> {
    if search_pattern.is_empty() {
        return vec![];
    }

    let file_matches = list_files(&path_g);
    let mut match_list: Vec<Match> = vec![];

    for file_match in &file_matches {
        let contents = match fs::read_to_string(file_match) {
            Ok(contents) => contents,
            Err(_) => {
                log::error!("Couldn't read file: {:?}", file_match.to_str());
                continue;
            }
        };

        for (i, s) in contents.match_indices(&search_pattern) {
            let (line_start, line) = match get_line(&contents, i) {
                Ok((line_start, line)) => (line_start, line),
                Err(_) => {
                    log::error!("Didn't find it the second time");
                    continue;
                }
            };

            match_list.push(Match {
                filepath: file_match.to_string_lossy().into_owned(),
                start: i - line_start,
                end: i - line_start + s.len(),
                line: line.into(),
                line_num: contents[..line_start].matches("\n").count(),
            });
        }
    }

    if !match_list.is_empty() {
        log::info!("{:?}", match_list);
    }

    match_list
}

fn get_line(contents: &str, index: usize) -> Result<(usize, &str), ()> {
    if index >= contents.len() {
        return Err(()); // Index out of bounds
    }

    // Find the start of the line by searching backwards for a newline character
    let line_start = contents[..index].rfind('\n').map_or(0, |pos| pos + 1);
    // Find the end of the line by searching forwards for a newline character
    let line_end = contents[index..]
        .find('\n')
        .map_or(contents.len(), |pos| index + pos);

    Ok((line_start, &contents[line_start..line_end]))
}

pub fn list_files(path_glob: &str) -> Vec<PathBuf> {
    glob(path_glob)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok) // Convert iterator of Result<PathBuf, glob::GlobError> to iterator of PathBuf, ignoring errors.
        .filter(|p| p.is_file()) // Keep only PathBufs that are files.
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use tempfile::{tempdir, TempDir};

    use super::*;

    fn create_temp_file(name: &String, dir: &TempDir) -> File {
        let file_path = dir.path().join(name);
        let file = File::create(file_path).expect("Failed to create temp file");
        file
    }

    #[test]
    fn test_list_files() {
        let tdir = tempdir().expect("failed to create temp dir");
        let afile = String::from("afile.txt");
        let f = create_temp_file(&afile, &tdir);
        let glob_path = format!("{:}/.*", tdir.path().to_str().expect("failed to get str"));
        let res = list_files(&glob_path);
        println!("{:?}", res);

        drop(f);
        tdir.close().expect("failed to close dir");
    }

    #[test]
    fn test_search() {
        let tdir = tempdir().expect("failed to create temp dir");
        let afile = String::from("afile.txt");
        let f = create_temp_file(&afile, &tdir);
        let glob_path = format!("{:}/.*", tdir.path().to_str().expect("failed to get str"));
        let res = search(glob_path, String::from(".*"));
        println!("{:?}", res);

        //let expected: Vec<String> = vec![tdir
        //    .path()
        //    .join(afile)
        //    .to_str()
        //    .expect("failed to convert path to str")
        //    .into()];
        //assert_eq!(res, expected);
        drop(f);
        tdir.close().expect("failed to close dir");
    }
}
