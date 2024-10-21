include!("declarations.rs");
include!("script_gen.rs");
// use std::fs;

// https://docs.rs/config-file/latest/config_file/

fn main() -> Result<(), u8>  {
    // open config file
    let mut config_file_name = String::new();
    let mut play_title = String::new();
    let mut play: Play = Vec::new();

    match parse_args(&mut config_file_name) {
        Ok(()) => {
            println!("Configuration file name: {}", config_file_name);
        },
        Err(_) =>  {
            eprintln!("Error: Bad command line arguments provided.");
            // std::process::exit(BAD_CMD_LINE);
            return Err(BAD_CMD_LINE);
        }
    }

    match script_gen(& config_file_name, &mut play_title, &mut play) {
        Ok(()) => {
            play.sort();
            recite(&play_title, &play);
        },
        Err(_) => {
            eprintln!("Error: Script Generation Failed.");
            return Err(FAILED_TO_GENERATE_SCRIPT);
        }
    }


    // return Ok(()) for success
    Ok(())
}

/// TODO Add function documentation, do this for everything in the future :)
fn parse_args(config_file_name: &mut String) -> Result<(), u8> {

    let mut args: Vec<String> = Vec::new();

    for arg in env::args() {
        args.push(arg);
    }

    if args.len() < MIN_ARGS || args.len() > MAX_ARGS || (args.len() == MAX_ARGS && args[OPT_WHINGE_POS] != "whinge") {
        usage(&args[PROG_NAME_POS]);
        return Err(BAD_CMD_LINE);
    }

    *config_file_name = args[CONFIG_POS].clone();

    if args.len() == MAX_ARGS {
        WHINGE_MODE.store(true, Ordering::SeqCst);
    }
    Ok(())
}

/// Prints a helpful usage message
fn usage(program_name: &String) {
    println!("usage: {} <configuration_file_name> [whinge]", program_name);
}

/// TODO Add function documentation, do this for everything in the future :)
fn recite(title: &String,  play: &Play) {

    println!("Title is: {}", title);

    // initialize variable for current character
    let mut current_character: Option<&CharName> = None;

    for line_tuple in play {
        match line_tuple {
            // REVIEW: what if character is an empty string? is that possible?
            (_, character,line ) if !character.is_empty() => {
                // do nothing if it's the same character as the current one
                if Some(character) != current_character {
                    println!();
                    println!("{}.", character); // print current character with "." after
                }
                current_character = Some(character); // update current_character
                println!("{}", line);
            }
            (..) => {
                return;
            }
        }
    }
}

