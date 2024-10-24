/// ADD FILE HEADER
///
///

use crate::lab2::declarations::{CharName, FAILED_TO_GENERATE_SCRIPT, WHINGE_MODE};
use crate::lab2::player::Player;
use crate::lab2::script_gen::{grab_trimmed_file_lines, CharacterTextFile,
                              CHARACTER_FILE_CONFIG_LINE_INDEX, CHARACTER_NAME_CONFIG_LINE_INDEX,
                              CONFIG_LINE_TOKENS, TITLE_LINE, CHARACTER_CONFIG_LINE};
use std::sync::atomic::Ordering;
use crate::lab2::scene_fragment::SceneFragment;



type ScriptConfig = Vec<(bool, String)>;
type Fragments = Vec<SceneFragment>;

pub struct Play {
    fragments: Fragments
}

impl Play {
    pub fn new() -> Play {
        Play {
            fragments: Vec::new()
        }
    }

    /// modified function for ScriptConfig bool parameters and SceneFragment types
    fn process_config(&mut self, script_config: ScriptConfig) -> Result<(), u8> { // I changed this to not be pub, hopefully that is fine
        let mut title = String::new();

        for config in script_config {
            match config {
                // if true, print title of new scene
                (true, new_title) => {
                    title = new_title;
                }
                // if false, use config_file name to push new SceneFragment into Play's vector
                (false, fragment_file_name) => {
                    let mut fragment = SceneFragment::new(&title);

                    if let Err(e) = fragment.prepare(&fragment_file_name) {
                        eprintln!("[X] ERROR: Failed to generate script from file: {}.",
                                  fragment_file_name);
                        return Err(FAILED_TO_GENERATE_SCRIPT);
                    }

                    self.fragments.push(fragment);
                    title = String::new();
                }
            }
        }
        Ok(())
    }

    /// TODO Add Documentation: question about constants
    // modified function for ScriptConfig to read in tokens and distinguish between scenes or another config file
    fn add_config(&self, config_line: &String, script_config: &mut ScriptConfig) {
        let config_line_tokens: Vec<&str> = config_line.split_whitespace().collect();
        // ignore blank lines
        if config_line_tokens.is_empty() {
            return;
        }
        // check if first line is [scene]
        if config_line_tokens[CHARACTER_NAME_CONFIG_LINE_INDEX] == "[scene]" {
            // if no more tokens, skip and whinge
            if config_line_tokens.len() == CHARACTER_FILE_CONFIG_LINE_INDEX {
                if WHINGE_MODE.load(Ordering::SeqCst) {
                    eprintln!("Missing scene title.")
                }
            }
            else {
                let scene_title = config_line_tokens[CHARACTER_FILE_CONFIG_LINE_INDEX..].join(" ");
                script_config.push((true, scene_title));
            }
        }
        else {
            let config_file_name = config_line_tokens[CHARACTER_NAME_CONFIG_LINE_INDEX].to_string();
            script_config.push((false, config_file_name));

            if config_line_tokens.len() >= CONFIG_LINE_TOKENS && WHINGE_MODE.load(Ordering::SeqCst) {
                eprintln!("Provided config line has the wrong number of tokens.");
            }
        }
    }

    // modified function for ScriptConfig to open script file an read lines
    fn read_config(&self, script_file_name: &String, script_config: &mut ScriptConfig) -> Result<(), u8> {
        let mut lines: Vec<String> = Vec::new();

        match grab_trimmed_file_lines(script_file_name, &mut lines) {
            Ok(()) => {
                if lines.is_empty() {
                    eprintln!("ERROR: Script file '{}' cannot be read", script_file_name);
                    return Err(FAILED_TO_GENERATE_SCRIPT);
                }

                for line in lines {
                    self.add_config(&line, script_config);
                }
                Ok(())
            }
            Err(..) => {
                eprintln!("ERROR: Failed to open or read script file '{}'", script_file_name);
                Err(FAILED_TO_GENERATE_SCRIPT)
            }
        }
    }

    // modified function for ScriptConfig to call read_config and check for fragment title
    pub fn prepare(&mut self, config_file_name: &String) -> Result<(), u8> {
        let mut script_config: ScriptConfig = vec![];

        match self.read_config(config_file_name, &mut script_config) {
            Ok(()) => {
                match self.process_config(script_config) {
                    Ok(()) => {
                        // check for fragments and title
                        if !self.fragments.is_empty() && !self.fragments[CHARACTER_NAME_CONFIG_LINE_INDEX].title.trim().is_empty() {
                            Ok(())
                        }
                        else  {
                            eprintln!("ERROR: First scene fragment has no title");
                            Err(FAILED_TO_GENERATE_SCRIPT)
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    // the enter and exit functions are not being accessed?? Made public if that's ok?
    pub fn recite(&mut self) {
        if self.fragments.is_empty() {
            eprintln!("ERROR: No scene fragments");
            return;
        }
        for i in 0..self.fragments.len() {
            let fragment = &mut self.fragments[i];

            if i == 0 {
            fragment.enter_all()
            }
            else {
                let previous_fragment = &self.fragments[i - 1];
                fragment.enter(previous_fragment);
            }
            fragment.recite();
            if i == self.fragments.len() -1 {
                fragment.exit_all();
            }
            else {
                let next_fragment = &self.fragments[i + 1];
                fragment.exit(next_fragment);
            }
        }
    }
}
