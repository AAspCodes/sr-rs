use core::fmt;
use glob::glob;
use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct Match {
    filepath: String,
    start: usize,
    end: usize,
    line: String,
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

pub fn sample_text() -> String {
    "Hello World".into()
}

#[allow(dead_code)]
pub fn search(path_g: String, search_pattern: String) -> Vec<Match> {
    let file_matches = list_files(path_g.as_str());
    let mut match_list: Vec<Match> = vec![];
    for file_match in file_matches {
        let contents = fs::read_to_string(file_match.clone()).expect("couldn't read file");
        let matches: Vec<(usize, &str)> = contents.match_indices(&search_pattern).collect();

        for (i, _) in matches {
            let (start, end, line) =
                get_line(&contents, i).expect("didn't find it the second time");
            match_list.push(Match {
                filepath: file_match.clone().to_string_lossy().into(),
                start,
                end,
                line: line.into(),
            });
        }
    }
    match_list
}

fn get_line(contents: &str, index: usize) -> Option<(usize, usize, &str)> {
    if index >= contents.len() {
        return None; // Index out of bounds
    }

    // Find the start of the line by searching backwards for a newline character
    let start = contents[..index].rfind('\n').map_or(0, |pos| pos + 1);

    // Find the end of the line by searching forwards for a newline character
    let end = contents[index..]
        .find('\n')
        .map_or(contents.len(), |pos| index + pos);

    Some((start, end, &contents[start..end]))
}

pub fn list_files(path_glob: &str) -> Vec<PathBuf> {
    glob(path_glob) // todo pass &str to list_files instead
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
