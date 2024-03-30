use glob::glob;
use std::{fs, path::PathBuf};

use crate::match_struct::Match;

pub fn search(path_g: String, search_pattern: String) -> Vec<Match> {
    if search_pattern.is_empty() {
        return vec![];
    }

    let file_matches = list_files(&path_g);
    let mut match_list: Vec<Match> = vec![];

    for file_match in &file_matches {
        if let Ok(contents) = read_file_contents(file_match) {
            match_list.append(&mut find_matches_in_file(
                &contents,
                &search_pattern,
                file_match,
            ));
        }
    }

    if !match_list.is_empty() {
        log::info!("{:?}", match_list);
    }

    match_list
}

fn read_file_contents(file_path: &PathBuf) -> Result<String, ()> {
    match fs::read_to_string(file_path) {
        Ok(contents) => Ok(contents),
        Err(_) => {
            log::error!("Couldn't read file: {:?}", file_path.to_str());
            Err(())
        }
    }
}

fn find_matches_in_file(contents: &str, search_pattern: &str, file_path: &PathBuf) -> Vec<Match> {
    let mut matches = vec![];

    for (i, s) in contents.match_indices(search_pattern) {
        let (line_start, line) = match get_line(contents, i) {
            Ok((line_start, line)) => (line_start, line),
            Err(_) => {
                log::error!("Didn't find it the second time");
                continue;
            }
        };

        matches.push(Match::new(
            file_path.to_string_lossy().into_owned(),
            i - line_start,
            i - line_start + s.len(),
            line.into(),
            contents[..line_start].matches("\n").count(),
        ));
    }

    matches
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
    match glob(path_glob) {
        Ok(paths) => paths
            .filter_map(Result::ok) // Convert iterator of Result<PathBuf, glob::GlobError> to iterator of PathBuf, ignoring errors.
            .filter(|p| p.is_file()) // Keep only PathBufs that are files.
            .collect(),
        Err(e) => {
            log::error!("Failed to read glob pattern: {}", e);
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_list_files_valid_pattern() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        let files = list_files(&format!("{}/*.txt", dir.path().to_string_lossy()));
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], file_path);
    }

    #[test]
    fn test_list_files_invalid_pattern() {
        let files = list_files("invalid_pattern[");
        assert!(files.is_empty());
    }

    #[test]
    fn test_get_line_valid_index() {
        let contents = "Hello\nWorld\n!";
        let (line_start, line) = get_line(contents, 6).unwrap();
        assert_eq!(line_start, 6);
        assert_eq!(line, "World");
    }

    #[test]
    fn test_get_line_index_out_of_bounds() {
        let contents = "Hello\nWorld\n!";
        let result = get_line(contents, 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_line_at_newline() {
        let contents = "Hello\nWorld\n!";
        let (line_start, line) = get_line(contents, 5).unwrap();
        assert_eq!(line_start, 0);
        assert_eq!(line, "Hello");
    }

    #[test]
    fn test_read_file_contents() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        let contents = read_file_contents(&file_path);
        assert_eq!(contents.unwrap(), "Hello, world!\n");
    }
}
