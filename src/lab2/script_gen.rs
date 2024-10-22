
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::atomic::Ordering;
use crate::lab2::declarations::*;
use crate::lab2::play::Play;

pub type CharacterTextFile = String;

// line numbers in character config files
static TITLE_LINE: usize = 0;
static CHARACTER_CONFIG_LINE: usize = 1;

// token indices and number of tokens
static CHARACTER_NAME_CONFIG_LINE_INDEX: usize = 0;
static CHARACTER_FILE_CONFIG_LINE_INDEX: usize = 1;
static CONFIG_LINE_TOKENS: usize = 2;

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

// reads character scripts from files in play_config and appends the lines
// associated with corresponding character to Play vector,
// if file fails to be processed, return ERROR.
fn process_config(play: &mut Play, play_config: &PlayConfig) -> Result<(), u8> {
    for config in play_config {
        let mut file_lines_ref: Vec<String> = Vec::new();

        // match tuple in play_config to destructure
        match config {
            (char_name, character_text_file) => {
                // try to get trimmed lines from file
                if let Err(..) = grab_trimmed_file_lines(character_text_file, &mut file_lines_ref) {
                    return Err(FAILED_TO_GENERATE_SCRIPT);
                }
                for line in &file_lines_ref {
                    add_script_line(play, line, char_name);
                }
            }
        }
    }
    Ok(())
}

// takes in a line from a config file,
// then pushes the tokens to the PlayConfig variable that is passed in by reference
fn add_config(config_line: &String, play_config: &mut PlayConfig) {
    let config_line_tokens: Vec<&str> = config_line.split_whitespace().collect();

    if config_line_tokens.len() != CONFIG_LINE_TOKENS {
        if WHINGE_MODE.load(Ordering::SeqCst) {
            eprintln!("Provided config line has the wrong number of tokens.");
        }
    }

    if config_line_tokens.len() >= CONFIG_LINE_TOKENS {
        play_config.push((
            config_line_tokens[CHARACTER_NAME_CONFIG_LINE_INDEX].to_string(),
            config_line_tokens[CHARACTER_FILE_CONFIG_LINE_INDEX].to_string()
        ))
    }
}

/*
 goes through a config file and if it doesn't return an ERROR,
 sets the play_title variable that is passed by reference,
 then adds all lines to the play_config variable that is passed by reference.
*/
fn read_config(config_file_name: &String, play_title: &mut String,
               play_config: &mut PlayConfig) -> Result<(), u8> {
    let mut lines: Vec<String> = Vec::new();

    match grab_trimmed_file_lines(config_file_name, &mut lines) {
        Ok(()) =>
            {
                // return error if not enough lines to generate the script
                if lines.len() <= CHARACTER_CONFIG_LINE { return Err(FAILED_TO_GENERATE_SCRIPT); }

                *play_title = lines[TITLE_LINE].clone();

                // adding the remaining lines to the play configuration data structure
                Ok(for line in &lines[1..] {
                    add_config(line, play_config)
                })
            }
        Err(..) => Err(FAILED_TO_GENERATE_SCRIPT)
    }
}

pub fn script_gen(config_file_name: &String, mut play_title: &mut String,
                         play: &mut Play) -> Result<(), u8> {
    let mut play_config: PlayConfig = vec![];

    match read_config(config_file_name, &mut play_title, &mut play_config) {
        Ok(()) => match process_config(play, &play_config) {
            Ok(()) => Ok(()),
            Err(..) => Err(FAILED_TO_GENERATE_SCRIPT)
        },
        Err(..) => Err(FAILED_TO_GENERATE_SCRIPT)
    }
}
