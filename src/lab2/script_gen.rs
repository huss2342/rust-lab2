/// ADD FILE HEADER
///
///

use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::sync::atomic::Ordering;
use crate::lab2::declarations::*;
// use crate::lab2::play::Play;

pub type CharacterTextFile = String;

// line numbers in character config files
pub static TITLE_LINE: usize = 0;
pub static CHARACTER_CONFIG_LINE: usize = 1;

// token indices and number of tokens
pub static CHARACTER_NAME_CONFIG_LINE_INDEX: usize = 0;
pub static CHARACTER_FILE_CONFIG_LINE_INDEX: usize = 1;
pub static CONFIG_LINE_TOKENS: usize = 2;

pub(crate) fn grab_trimmed_file_lines(file_name: &String, file_lines: &mut Vec<String>) -> Result<(), u8> {
    /*
        found this from here because I was having a syntax issue
        https://users.rust-lang.org/t/rust-file-open-error-handling/50681
    */
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("ERROR: Failed to open file '{}': {}", file_name, e);
            return Err(FAILED_TO_GENERATE_SCRIPT);
        }
    };

    // https://doc.rust-lang.org/std/io/struct.BufReader.html
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => return Ok(()), // indicates success
            Ok(..) => file_lines.push(line.trim().to_string()),
            Err(e) => {
                eprintln!("ERROR: Failed to read line '{}': {}", file_name, e);
                return Err(FAILED_TO_GENERATE_SCRIPT);
            }
        }
    }
}


