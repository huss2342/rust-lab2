/// ADD FILE HEADER
///
///

pub mod lab2;
use std::env;
use std::sync::atomic::Ordering;
use lab2::declarations::*;
use lab2::play::Play;
use lab2::return_wrapper::ReturnWrapper;


fn main() -> ReturnWrapper {
    // open script config file
    let mut script_file_name = String::new();
    let mut play_scene = String::new(); // TODO why this no use?

    match parse_args(&mut script_file_name) {
        Ok(()) => {
            let mut play = Play::new();
            match play.prepare(&script_file_name) {
                Ok(()) => {
                    play.recite();
                },
                Err(..) => return ReturnWrapper::new(Err(FAILED_TO_GENERATE_SCRIPT)),
            }
        },
        Err(..) => {
            eprintln!("ERROR: Bad command line arguments provided.");
            return ReturnWrapper::new(Err(BAD_CMD_LINE))
        }
    }
    ReturnWrapper::new(Ok(()))

    // FIXME what's the commented code here  -Nick
    // match script_gen(&config_file_name, &mut play_title, &mut play) {
    //     Ok(()) => {
    //         play.sort();
    //         recite(&play_title, &play);
    //         Ok(())
    //     },
    //     Err(..) => {
    //         eprintln!("ERROR: Script Generation Failed.");
    //         Err(FAILED_TO_GENERATE_SCRIPT)
    //     }
    // }
}

fn parse_args(script_file_name: &mut String) -> Result<(), u8> {
    // let mut args: Vec<String> = Vec::new();
    // TODO if this is meant to be mut it's probably for a reason so we should make sure we are using it right
    let mut args: Vec<String> = env::args().collect();

    // for arg in env::args() {
    //     args.push(arg);
    // }

    if args.len() < MIN_ARGS || args.len() > MAX_ARGS ||
        (args.len() == MAX_ARGS && args[OPT_WHINGE_POS] != "whinge") {
        usage(&args[SCRIPT_NAME_POS]);
        return Err(BAD_CMD_LINE);
    }

    *script_file_name = args[SCRIPT_FILE_POS].clone();

    if args.len() == MAX_ARGS {
        WHINGE_MODE.store(true, Ordering::SeqCst);
    }
    Ok(())
}

// Prints a helpful usage message
fn usage(script_name: &String) {
    println!("usage: {} <script_file_name> [whinge]", script_name);
}

