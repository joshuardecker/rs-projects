mod read_directory;
mod error;

use crate::read_directory::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path;
    if args.len() < 2 {
        path = ".";
    } else {
        path = args[1].as_str();
    }
    
    print_directory(path).unwrap();
}
