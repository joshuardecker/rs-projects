mod error;
mod replace;

use std::env;
use crate::replace::get_colorzied_file_lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        panic!("Please provide grep with enough arguments!");
    }
    
    let keyword = &args[1];
    let file_path = args[2].as_str();

    let colored_lines_result = get_colorzied_file_lines(keyword.clone(), file_path);

    let colored_lines = match colored_lines_result {
        Ok(colored_lines) => colored_lines,
        Err(error) => panic!("{:?}", error)
    };

    print!("{}", colored_lines);
}
