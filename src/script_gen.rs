// TODO: might be able to remove these two imports  below?

use std::fs::File;
use std::io::{BufReader, BufRead};

type CharacterTextFile = String;
type PlayConfig = Vec<(CharName, CharacterTextFile)>;

// line numbers in character config files
static TITLE_LINE : usize = 0;
static CHARACTER_CONFIG_LINE : usize = 1;

// token indices and number of tokens
static CHARACTER_NAME_CONFIG_LINE_INDEX : usize = 0;
static CHARACTER_FILE_CONFIG_LINE_INDEX : usize = 1;
static CONFIG_LINE_TOKENS : usize = 2;


// TODO Add function documentation, do this for everything in the future :)
fn add_script_line(play: &mut Play, line: &String, char_part_name: &String) {
    if line.is_empty() { return }

    let Some((first_token, rest_of_line)) = line.split_once(char::is_whitespace) else {
        return // leave if split_once returns None
    };

    // TODO I'm a bit unsure of if shadowing is a good idea here
    let first_token = first_token.trim();
    let rest_of_line = rest_of_line.trim();

    // match the result of parsing and if successful, push the line into the play
    match first_token.parse::<usize>() {
        // REVIEW: might need to do .clone() instead of .to_string() here?
        Ok(line_num) =>
            play.push((line_num, char_part_name.to_string(), rest_of_line.to_string())),
        Err(_) => if WHINGE_MODE.load(Ordering::SeqCst) {
            eprintln!("[X] ERROR: The token \"{}\" does not represent a valid usize value.",
                      first_token);
        },
    }
}

// TODO Add function documentation, do this for everything in the future :)
fn grab_trimmed_file_lines(file_name: &String, file_lines: &mut Vec<String>) -> Result<(), u8> {
    /*
        found this from here because I was having a syntax issue
        https://users.rust-lang.org/t/rust-file-open-error-handling/50681
    */
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("[X] ERROR: Failed to open file '{}': {}", file_name, e);
            return Err(2) // FIXME with a constant later
        }
    };

    // https://doc.rust-lang.org/std/io/struct.BufReader.html
    let mut reader= BufReader::new(file);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => return Ok(()), // indicates success
            // REVIEW: is it possible that we push an empty line here?
            Ok(_) => file_lines.push(line.trim().to_string()),
            Err(e) => {
                eprintln!("[X] ERROR: Failed to read line '{}': {}", file_name, e);
                return Err(3) // FIXME with a constant later
            }
        }

    }
}

// TODO Add function documentation, do this for everything in the future :)
fn process_config(play: &mut Play, play_config: &PlayConfig) -> Result<(), u8>  {



    for config in play_config {
        let mut file_lines_ref: Vec<String> = Vec::new();

        // match tuple in play_config to destructure
        match config {
            (char_name, character_text_file) => {
                // try to get trimmed lines from file
                if let Err(_) = grab_trimmed_file_lines(character_text_file, &mut file_lines_ref) {
                    return Err(2) // FIXME with a constant later
                }
                for line in &file_lines_ref {
                    add_script_line(play, line, char_name);
                }
            }
        }
    }
    Ok(())
}

/// takes in a line from a config file,
/// then pushes the tokens to the PlayConfig variable that is passed in by reference
fn add_config(config_line: &String, play_config: &mut PlayConfig) {
    let config_line_tokens: Vec<&str> = config_line.split_whitespace().collect();

    if config_line_tokens.len() != CONFIG_LINE_TOKENS {
        if WHINGE_MODE.load(Ordering::SeqCst) {
            eprintln!("Provided config line has the wrong number of tokens.");
        }
    }

    if config_line_tokens.len() >= CONFIG_LINE_TOKENS {
        // you just have to make it beautiful 🥰
        play_config.push((
             config_line_tokens[CHARACTER_NAME_CONFIG_LINE_INDEX].to_string(),
             config_line_tokens[CHARACTER_FILE_CONFIG_LINE_INDEX].to_string()
        ));
    }
}

/**
* goes through a config file and if it doesn't return an error,
* sets the play_title variable that is passed by reference,
* then adds all lines to the play_config variable that is passed by reference.
*/
fn read_config(config_file_name: &String, play_title: &mut String,
                play_config: &mut PlayConfig) -> Result<(), u8> {
    let mut lines: Vec<String> = Vec::new();

    match grab_trimmed_file_lines(config_file_name, &mut lines) {
        Ok(()) =>
            {
                // return error if not enough lines to generate the script
                if lines.len() <= CHARACTER_CONFIG_LINE { return Err(FAILED_TO_GENERATE_SCRIPT) }

                // storing the first line into the string for the title of the play
                *play_title = lines[TITLE_LINE].clone();

                // adding the remaining lines to the play configuration data structure
                Ok(for line in &lines[1..] {
                    add_config(line, play_config)
                })
            },
        Err(_) => return Err(FAILED_TO_GENERATE_SCRIPT)
    }
}

fn script_gen(config_file_name: &String, mut play_title: &mut String,
                play: &mut Play) -> Result<(), u8> {
    let mut play_config: PlayConfig = vec![];

    match read_config(config_file_name, &mut play_title, &mut play_config){
        Ok(()) => match process_config(play, &play_config)  {
            Ok(()) => Ok(()),
            Err(_) => Err(2) // FIXME return an error indicating that script gen failed
        },
        Err(_)  => Err(1) // FIXME script generation failed error
    }
}