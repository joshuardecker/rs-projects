use core::fmt;
use std::ffi::OsString;
use std::fs::{self, DirEntry};
use crate::error::Error;
use colored::Colorize;

struct UnParsedDirectory {
    sub_directories: Vec<OsString>,
    files: Vec<OsString>,
    sys_links: Vec<OsString>,
}

// Same thing as the UnParsed Directory,
// just instead has been converted to utf8 strings.
struct ParsedDirectory {
    sub_directories: Vec<String>,
    files: Vec<String>,
    sys_links: Vec<String>,
}

pub fn print_directory(path: &str) -> Result<(), Error> {
    let dir = get_directory(path)?;

    let parsed_dir = dir.parse()?;

    print!("{}", parsed_dir);

    Ok(())
}

impl UnParsedDirectory {
    fn new() -> Self {
        UnParsedDirectory{sub_directories: Vec::new(), files: Vec::new(), sys_links: Vec::new()}
    }
    
    fn parse(self) -> Result<ParsedDirectory, Error> {
        let mut directories = Vec::new();
        let mut files = Vec::new();
        let mut sys_links = Vec::new();
        
        for dir in self.sub_directories {
            match dir.into_string() {
                Ok(dir_string) => directories.push(dir_string),
                Err(_) => return Err(Error::ConvertOSStringError)
            }
        }

        for file in self.files {
            match file.into_string() {
                Ok(file_string) => files.push(file_string),
                Err(_) => return Err(Error::ConvertOSStringError)
            }
        }

        for sys_link in self.sys_links {
            match sys_link.into_string() {
                Ok(sys_link_string) => sys_links.push(sys_link_string),
                Err(_) => return Err(Error::ConvertOSStringError)
            }
        }

        Ok(ParsedDirectory{sub_directories: directories, files: files, sys_links: sys_links})
    }

    fn add_item(&mut self, item: DirEntry) -> Result<(), Error>{
        let item_type = match item.file_type() {
            Ok(item_type) => item_type,
            Err(error) => return Err(Error::GetFileTypeError(
                error.to_string())
            )
        };

        if item_type.is_dir() {
            self.sub_directories.push(item.file_name());
        } else if item_type.is_symlink() {
            self.sys_links.push(item.file_name());
        } else {
            self.files.push(item.file_name());
        }

        Ok(())
    }
}

impl fmt::Display for ParsedDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dirs_string = self.sub_directories.join("  ");
        let mut files_string = self.files.join("  ");
        let mut sys_links_string = self.sys_links.join("  ");

        // Adds a line break for every section.
        // That way if a directory lets say has onyl files, but
        // no sub directories or sys_links, there is no
        // unnessesary line breaks.
        if dirs_string.len() > 0 {
            dirs_string += "\n";
        }
        if files_string.len() > 0 {
            files_string += "\n";
        }
        if sys_links_string.len() > 0 {
            sys_links_string += "\n";
        }

        write!(f, "{}{}{}", 
            dirs_string.green(), 
            files_string.red(), 
            sys_links_string.blue()
        )
    }
}

fn get_directory(path: &str) -> Result<UnParsedDirectory, Error> {
    let dir_result = fs::read_dir(path);

    let dir_entries = match dir_result {
        Ok(dir_entries) => dir_entries,
        Err(error) => return Err(Error::ReadDirectoryError(
            error.to_string())
        )
    };

    let mut unparsed_dir = UnParsedDirectory::new();

    // It is unknown whether this item's name can be given to the program.
    // If it succeeds, the name is saved, if not the function fails.
    for item in dir_entries {
        let dir_entry = match item {
            Ok(dir_entry) => dir_entry,
            Err(error) => return Err(Error::ConvertFileNameError(
                error.to_string())
            )
        };

        if let Err(error) = unparsed_dir.add_item(dir_entry) {
            return Err(error);
        }
    }

    Ok(unparsed_dir)
}