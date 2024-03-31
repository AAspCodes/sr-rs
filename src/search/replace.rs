use std::fs;
use std::io::Write;

use crate::app::App;
use crate::enums::input_enums::InputBox;
use crate::models::match_struct::Match;

use super::search;

pub fn replace(app: &App) -> std::io::Result<()> {
    let search_pattern = app.input[InputBox::Search.pos()].value().to_string();
    let replacement = app.input[InputBox::Replace.pos()].value().to_string();
    let search_glob = app.input[InputBox::Filepath.pos()].value().to_string();
    let res: Vec<Match> = search(search_glob, search_pattern);
    let matches = res
        .iter()
        .map(|m| {
            let mut temp = m.clone();
            temp.set_replacement(replacement.clone());
            temp
        })
        .collect::<Vec<Match>>();
    replace_matches(&matches)
}

// fn replace_matches(matches: &[Match]) -> std::io::Result<()> {
//     for m in matches {
//         let mut contents = fs::read_to_string(&m.get_filepath())?;
//         let new_contents = contents.replace(&m.get_line(), &m.get_replacement());
//         let mut file = fs::OpenOptions::new()
//             .write(true)
//             .truncate(true)
//             .open(&m.get_filepath())?;
//         file.write_all(new_contents.as_bytes())?;
//     }
//     Ok(())
// }

use std::collections::HashMap;

fn replace_matches(matches: &[Match]) -> std::io::Result<()> {
    // Group the matches by file
    let mut matches_by_file: HashMap<String, Vec<Match>> = HashMap::new();
    for m in matches {
        matches_by_file
            .entry(m.get_filepath().to_string())
            .or_default()
            .push(m.clone());
    }

    // Process each file
    for (filepath, matches) in matches_by_file {
        let mut new_contents = fs::read_to_string(&filepath)?;

        // Sort the matches by their start indices in descending order
        let mut sorted_matches = matches;
        sorted_matches
            .sort_unstable_by(|a, b| b.get_file_index_start().cmp(&a.get_file_index_start()));

        // Replace the matches from the end of the string towards the beginning
        for match_ in sorted_matches {
            new_contents = new_contents[..match_.get_file_index_start()].to_string()
                + &match_.get_replacement()
                + &new_contents[match_.get_file_index_start() + match_.get_match_length()..];
        }

        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&filepath)?;
        file.write_all(new_contents.as_bytes())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::app::App;
    use crate::enums::input_enums::InputBox;
    use std::fs::File;
    use std::io::Write;

    fn set_input_value(app: &mut App, input_box: InputBox, value: String) {
        let input = app.input.get(input_box.pos()).unwrap();
        app.input[input_box.pos()] = input.clone().with_value(value);
    }

    #[test]
    fn test_replace() -> std::io::Result<()> {
        // Setup: Create a test file with some content
        let test_file_path = "/tmp/test_file.txt";
        let mut file = File::create(test_file_path)?;
        writeln!(file, "Hello, world! Hello, world!")?;

        // Create a test App
        let mut app = App::default();
        set_input_value(&mut app, InputBox::Search, "world".into());
        set_input_value(&mut app, InputBox::Replace, "Rust".into());
        set_input_value(&mut app, InputBox::Filepath, test_file_path.into());

        // Call the function to test
        replace(&app)?;

        // Check that the file content has been replaced correctly
        let content = fs::read_to_string(test_file_path)?;
        assert_eq!(content, "Hello, Rust! Hello, Rust!\n");

        // Cleanup: Remove the test file
        fs::remove_file(test_file_path)?;

        Ok(())
    }
}
