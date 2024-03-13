use glob::glob;
use std::{process::Command, str};






pub struct matches {
    filepath: String,
    match_start: usize,
    match_end: usize,
    matching_line: String,
}



pub fn sample_text() -> String {
    "Hello World".into()
}

#[allow(dead_code)]
pub fn search(path_g: String, search_pattern: String) -> Vec<String> {
    let mut path_glob = String::from("./*");
    println!("tiger{}", path_g);
    if path_g != String::from("") {
        path_glob = path_g;
    }
    list_files(path_glob, search_pattern)
}

pub fn list_files(path_glob: String, search_pattern: String) -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    for p in glob(path_glob.as_str()).expect("glob failed") {
        match p {
            Ok(path) => ret.push(path.to_string_lossy().into_owned()),
            _ => {}
        }
    }
    // read files in ret, and get line found

    ret
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
        let res = list_files(glob_path, String::from("*"));
        println!("{:?}", res);

        let expected: Vec<String> = vec![tdir
            .path()
            .join(afile)
            .to_str()
            .expect("failed to convert path to str")
            .into()];
        assert_eq!(res, expected);
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
