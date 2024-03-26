use core::fmt;
use std::fs::File;
use std::io::Read;
use colored::Colorize;
use crate::error::Error;

pub struct ParsedLines {
    lines: Vec<String>
}

pub fn get_colorzied_file_lines(keyword: String, path: &str) -> Result<ParsedLines, Error>{
    let colored_keyword = keyword.red().to_string();
    
    let file_result = File::open(path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => return Err(Error::ReadFileError(
            error.to_string())
        )
    };

    let mut file_string = String::new();
    match file.read_to_string(&mut file_string) {
        Ok(_) => (),
        Err(error) => return Err(Error::FileToStringError(
            error.to_string())
        )
    }

    let file_lines = parse_file_to_lines(file_string, &keyword);
    let mut colored_file_lines = Vec::new();

    for file_line in file_lines {
        colored_file_lines.push(
            file_line.replace(&keyword, &colored_keyword)
        );
    }

    Ok(ParsedLines{lines: colored_file_lines})
}

// Take a whole file string, and convert it to a vec of its lines.
// Ignores any line that does not have the keyword.
fn parse_file_to_lines(file_string: String, keyword: &String) -> Vec<String> {
    let mut seperated_lines = Vec::new();

    for line in file_string.lines() {
        if !line.contains(keyword) {
            continue;
        }
        
        // Converts &str to String here because its easy.
        seperated_lines.push(line.to_string());
    }

    seperated_lines
}

impl fmt::Display for ParsedLines {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }
        
        write!(f, "")
    }
}