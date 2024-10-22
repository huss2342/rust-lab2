
pub mod lab2;

use std::env;
use std::sync::atomic::Ordering;
use lab2::declarations::*;
use lab2::play::Play;
use lab2::player::Player;
// use lab2::play::script_gen;


fn main() -> Result<(), u8> {
    // open config file
    let mut config_file_name = String::new();
    let mut play_title = String::new();

    match parse_args(&mut config_file_name) {
        Ok(()) => {
            let mut play = Play::new();
            match play.prepare(&config_file_name) {
                Ok(()) => {
                    play.recite();
                },
                Err(..) => ()
            }
        },
        Err(..) => {
            eprintln!("ERROR: Bad command line arguments provided.");
            return Err(BAD_CMD_LINE)
        }
    }
    Ok(())
    // match script_gen(&config_file_name, &mut play_title, &mut play) {
    //     Ok(()) => {
    //         play.sort();
    //         recite(&play_title, &play);
    //         Ok(())
    //     }
    //     Err(..) => {
    //         eprintln!("ERROR: Script Generation Failed.");
    //         Err(FAILED_TO_GENERATE_SCRIPT)
    //     }
    // }
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

