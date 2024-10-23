/// ADD FILE HEADER
///
///

use std::sync::atomic;
use std::cmp;
use crate::lab2::declarations::{FAILED_TO_GENERATE_SCRIPT, WHINGE_MODE};
use crate::lab2::script_gen::grab_trimmed_file_lines;

type PlayLines = Vec<(usize, String)>;

static FIRST_LINE: usize = 0;

pub struct Player {
    pub name: String,
    lines: PlayLines,
    index: usize
}


impl Ord for Player {
    /// TODO We should do some extensive testing to ensure this works correctly, add documentation
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.eq(other) {
            return cmp::Ordering::Equal;
        }

        // (1) if one player has lines and the other does not
        if self.lines.is_empty() && !other.lines.is_empty() {
            return cmp::Ordering::Less;
        }
        if !self.lines.is_empty() && other.lines.is_empty() {
            return cmp::Ordering::Greater;
        }

        // (2) both have lines, whoever speaks first is less
        let (self_line_num, _) = self.lines[FIRST_LINE];
        let (other_line_num, _) = other.lines[FIRST_LINE];
        if self_line_num < other_line_num {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        }
    }
}
impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        if self.lines.is_empty() && other.lines.is_empty() {
            return true;
        }
        let (self_line_num, _) = self.lines[FIRST_LINE];
        let (other_line_num, _) = other.lines[FIRST_LINE];
        if self_line_num == other_line_num {
            return true;
        }
        false
    }
}
impl Eq for Player {}

impl Player {
    /// TODO Add Documentation
    pub fn new(name: &String) -> Player {
        Player {
            name: name.clone(),
            lines: Vec::new(),
            index: 0
        }
    }

    /// TODO Add Documentation
    fn add_script_line(&mut self, line: &String) {
        if line.is_empty() { return; }

        let Some((first_token, rest_of_line)) = line.split_once(char::is_whitespace) else {
            // Badly formed line, no whitespace split
            if WHINGE_MODE.load(atomic::Ordering::SeqCst) {
                eprintln!("ERROR: The line '{}' is badly formed and will be skipped.", line)
            }
            return;
        };

        let first_token = first_token.trim();
        let rest_of_line = rest_of_line.trim();

        // match the result of parsing and if successful, push the line into the play
        match first_token.parse::<usize>() {
            Ok(line_num) =>
                self.lines.push((line_num, rest_of_line.to_string())),
            Err(..) => if WHINGE_MODE.load(atomic::Ordering::SeqCst) {
                eprintln!("ERROR: The token \"{}\" does not represent a valid usize value.",
                          first_token);
            }
        }
    }

    /// TODO Add Documentation
    pub fn prepare (&mut self, part_file_name: &String) -> Result<(), u8> {
        let mut file_lines_ref: Vec<String> = Vec::new();

        if let Err(..) = grab_trimmed_file_lines(part_file_name, &mut file_lines_ref) {
            return Err(FAILED_TO_GENERATE_SCRIPT);
        }

        for line in &file_lines_ref {
            self.add_script_line(line);
        }
        self.lines.sort();
        Ok(())
    }

    /// TODO Add Documentation
    pub fn speak(&mut self, last_speaker: &mut String) {
        if self.index >= self.lines.len() {
            return ()
        }
        if self.name != *last_speaker {
            *last_speaker = self.name.clone();
            println!("\n{}:", self.name);
        }
        println!("{}", self.lines[self.index].1);
        self.index += 1;
    }

    /// TODO Add Documentation
    pub fn next_line(&self) -> Option<usize> {
        if self.index < self.lines.len() { Some(self.lines[self.index].0) }
        else { None }
    }
}