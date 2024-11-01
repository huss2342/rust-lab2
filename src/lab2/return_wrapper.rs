
use std::process::{Termination, ExitCode};
use crate::lab2::declarations::SUCCESS;

pub struct ReturnWrapper {
    ret_code: u8,
}

impl ReturnWrapper {
    pub fn new(result: Result<(), u8>) -> ReturnWrapper {
        ReturnWrapper {
            ret_code: match result { Ok(_) => SUCCESS, Err(e) => e },
        }
    }
}

impl Termination for ReturnWrapper {
    fn report(self) -> ExitCode {
        if self.ret_code != 0 {
            eprintln!("Error: {}", self.ret_code);
        }
        ExitCode::from(self.ret_code)
    }
}

