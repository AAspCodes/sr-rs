use std::{process::Command, str};

pub fn sample_text() -> String {
    "Hello World".into()
}

#[allow(dead_code)]
pub fn search() {
    unimplemented!()
}

pub fn list_files(path_glob: String) -> Vec<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("find {} -type f", path_glob))
        .output()
        .expect("Failed to find files");
    let convert = |buf: &Vec<u8>| -> Vec<String> {
        let stdout_str = str::from_utf8(buf).expect("Failed to convert stdout to string");
        // Split the string slice on '\n' and collect into a Vec<String>
        let lines: Vec<String> = stdout_str.lines().map(String::from).collect();
        lines
    };
    let mut ret = convert(&output.stderr);
    ret.extend(convert(&output.stdout));
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
        let glob_path = format!("{:?}/*", tdir.path());
        let res = list_files(glob_path);
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
}