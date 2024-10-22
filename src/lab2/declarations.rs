use std::sync::atomic::AtomicBool;

pub type LineNum = usize;
pub type CharName = String;
pub type Line = String;
pub type LineTuple = (LineNum, CharName, Line);

// Minimum and Maximum number of arguments constants
pub static MIN_ARGS: usize = 2;
pub static MAX_ARGS: usize = 3;

// Command line argument position constants
pub static PROG_NAME_POS: usize = 0;
pub static CONFIG_POS: usize = 1;
pub static OPT_WHINGE_POS: usize = 2;

// Return value constants
pub static BAD_CMD_LINE: u8 = 1;
pub static FAILED_TO_GENERATE_SCRIPT: u8 = 2;

// AtomicBool to keep track of if we are whinging
pub static WHINGE_MODE: AtomicBool = AtomicBool::new(false);