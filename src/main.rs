
pub mod lab2;

use std::env;
use std::sync::atomic::Ordering;
use lab2::declarations::*;
use lab2::play::Play;
use lab2::script_gen::script_gen;


fn main() -> Result<(), u8> {
    // open config file
    let mut config_file_name = String::new();
    let mut play_title = String::new();
    let mut play: Play = Vec::new();

    match parse_args(&mut config_file_name) {
        Ok(()) => { }
        Err(..) => {
            eprintln!("ERROR: Bad command line arguments provided.");
            return Err(BAD_CMD_LINE);
        }
    };

    match script_gen(&config_file_name, &mut play_title, &mut play) {
        Ok(()) => {
            play.sort();
            recite(&play_title, &play);
            Ok(())
        }
        Err(..) => {
            eprintln!("ERROR: Script Generation Failed.");
            Err(FAILED_TO_GENERATE_SCRIPT)
        }
    }
}

fn parse_args(config_file_name: &mut String) -> Result<(), u8> {
    let mut args: Vec<String> = Vec::new();

    for arg in env::args() {
        args.push(arg);
    }

    if args.len() < MIN_ARGS || args.len() > MAX_ARGS ||
        (args.len() == MAX_ARGS && args[OPT_WHINGE_POS] != "whinge") {
        usage(&args[PROG_NAME_POS]);
        return Err(BAD_CMD_LINE);
    }

    *config_file_name = args[CONFIG_POS].clone();

    if args.len() == MAX_ARGS {
        WHINGE_MODE.store(true, Ordering::SeqCst);
    }
    Ok(())
}

// Prints a helpful usage message
fn usage(program_name: &String) {
    println!("usage: {} <configuration_file_name> [whinge]", program_name);
}


// Iterates through Play vector, prints the title, tracks the current character,
// prints character names when they change, and outputs corresponding dialogue.
fn recite(title: &String, play: &Play) {
    println!("{}", title);

    let mut current_character: Option<&CharName> = None;

    // used for reference:
    // https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_tuple.html
    for line_tuple in play {
        match line_tuple {
            (_, character, line) => {
                // if the character changes
                if Some(character) != current_character {
                    println!();
                    println!("{}.", character);
                }
                current_character = Some(character);
                println!("{}", line);
            }
        }
    }
}
