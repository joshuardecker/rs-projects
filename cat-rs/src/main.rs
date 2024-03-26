mod read_file;
mod error;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        panic!("Please provide a file to to output!");
    }

    let file_string_result = read_file::read_file_to_string(&args[1]);

    match file_string_result {
        Ok(file_string) => println!("{}", file_string),
        Err(error) => panic!("{:?}", error.unwrap()),
    }
}
