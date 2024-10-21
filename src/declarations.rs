use std::env;
use std::sync::atomic::{AtomicBool, Ordering};

type LineNum = usize;
type CharName = String;
type Line = String;
type LineTuple = (LineNum, CharName, Line);
type Play = Vec<LineTuple>;


// Minimum and Maximum number of arguments constants
static MIN_ARGS: usize = 2;
static MAX_ARGS: usize = 3;

// Command line argument position constants
static PROG_NAME_POS: usize = 0;
static CONFIG_POS: usize = 1;
static OPT_WHINGE_POS: usize = 2;

// Return value constants
static BAD_CMD_LINE: u8 = 1;
static FAILED_TO_GENERATE_SCRIPT: u8 = 2;


// AtomicBool to keep track of if we are whinging
static WHINGE_MODE: AtomicBool = AtomicBool::new(false);