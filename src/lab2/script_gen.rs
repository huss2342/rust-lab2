
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::lab2::declarations::*;

pub type CharacterTextFile = String;

// line numbers in character config files
pub static TITLE_LINE: usize = 0;
pub static CHARACTER_CONFIG_LINE: usize = 1;

// token indices and number of tokens
pub static CHARACTER_NAME_CONFIG_LINE_INDEX: usize = 0;
pub static CHARACTER_FILE_CONFIG_LINE_INDEX: usize = 1;
pub static CONFIG_LINE_TOKENS: usize = 2;

// documentation example taken from:
// https://deterministic.space/machine-readable-inline-markdown-code-cocumentation.html#markdown-formatting-conventions

///
/// Fills a vector with the lines in a file with the given name.
///
/// # Parameters
///
/// - `file_name`: A reference to a String that holds the name of the file to read
/// - `file_lines`: A mutable reference to a vector where the lines will be added
///
/// # Returns
///
/// A `Result` which is:
///
/// - `Ok`: The file was successfully read and inputted into `file_lines`
/// - `Err`: A `u8` value representing an error: unable to generate the script.
///     This can occur if the file could not be opened or the function `read_line`
///     returned an error while trying to read a line from the file.
///
pub fn grab_trimmed_file_lines(file_name: &String, file_lines: &mut Vec<String>) -> Result<(), u8> {

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


