use crate::lab2::declarations::{CharName, FAILED_TO_GENERATE_SCRIPT, WHINGE_MODE};
use crate::lab2::player::Player;
use crate::lab2::script_gen::{grab_trimmed_file_lines, CharacterTextFile,
                              CHARACTER_FILE_CONFIG_LINE_INDEX, CHARACTER_NAME_CONFIG_LINE_INDEX,
                              CONFIG_LINE_TOKENS, CHARACTER_CONFIG_LINE};
use std::sync::atomic::Ordering;

type PlayConfig = Vec<(CharName, CharacterTextFile)>;

pub struct SceneFragment {
    // made public if that's ok
    pub title: String,
    players: Vec<Player>,
}

impl SceneFragment {

    /// changed parameter to reference to string and cloned string
    pub fn new(title: &String) -> SceneFragment {
        SceneFragment {
            title: title.clone(),
            players: Vec::new(),
        }
    }

    ///
    /// Print a message for every player that needs to enter for the next scene.
    /// Do this after printing out the title of the scene if applicable.
    ///
    /// # Parameters
    ///
    /// - `self`: A reference to self
    /// - `next`: A reference to another instance of the struct SceneFragment
    ///
    pub fn enter(&self, next: &SceneFragment) {
        // check to see if title contains only whitespace. If not, prints out scene title
        if !self.title.trim().is_empty() {
            println!(); // print a newline first to make the printout cleaner
            println!("{}", self.title);
        }

        for next_player in &next.players {
            // determine if the previous scene contains the player from the next scene
            let mut contains = false;
            for player in &self.players {
                if player.name == next_player.name {
                    contains = true;
                }
            }

            if !contains {
                println!("[Enter {}.]", next_player.name);
            }
        }

    }

    ///
    /// Print a message for every player in this scene stating that they are entering.
    /// Do this after printing out the title of the scene if applicable.
    ///
    /// # Parameters
    ///
    /// - `self`: A reference to self
    ///
    pub fn enter_all(&self) {
        // check to see if title contains only whitespace. If not, prints out scene title
        if !self.title.trim().is_empty() {
            println!(); // print a newline first to make the printout cleaner
            println!("{}", self.title);
        }

        for player in &self.players {
            println!("[Enter {}.]", player.name);
        }
    }

    ///
    /// Print a message for every player that isn't in the next scene to exit.
    ///
    /// # Parameters
    ///
    /// - `self`: A reference to self
    /// - `next`: A reference to another instance of the struct SceneFragment
    ///
    pub fn exit(&self, next: &SceneFragment) {
        for player in self.players.iter().rev() {
            // determine if the next scene contains the player from the previous scene
            let mut contains = false;
            for next_player in &next.players {
                if player.name == next_player.name {
                    contains = true;
                }
            }
            if !contains {
                println!("[Exit {}.]", player.name);
            }
        }
    }

    ///
    /// Print a message for every player in this scene stating that they are exiting.
    ///
    /// # Parameters
    ///
    /// - `self`: A reference to self
    ///
    pub fn exit_all(&self) {
        for player in self.players.iter().rev() {
            println!("[Exit {}.]", player.name);
        }
    }


    /// create the players vector
    fn process_config(&mut self, play_config: PlayConfig) -> Result<(), u8> {
        for config in play_config {
            match config {
                (char_name, part_file_name) => {
                    let mut player = Player::new(&char_name);
                    if let Err(e) = player.prepare(&part_file_name) {
                        eprintln!("ERROR: Failed to generate script for character {}.",
                                  char_name);
                        return Err(e);
                    }
                    self.players.push(player);
                }
            }
        }
        self.players.sort();
        Ok(())
    }

    /// add a config file to the scene
    fn add_config(&self, config_line: &String, play_config: &mut PlayConfig) {
        let config_line_tokens: Vec<&str> = config_line.split_whitespace().collect();

        if config_line_tokens.len() != CONFIG_LINE_TOKENS {
            if WHINGE_MODE.load(Ordering::SeqCst) {
                eprintln!("Provided config line has the wrong number of tokens.");
            }
        }

        if config_line_tokens.len() >= CONFIG_LINE_TOKENS {
            play_config.push((
                config_line_tokens[CHARACTER_NAME_CONFIG_LINE_INDEX].to_string(),
                config_line_tokens[CHARACTER_FILE_CONFIG_LINE_INDEX].to_string()
            ))
        }
    }

    /// read from the config file
    fn read_config(&self, config_file_name: &String, play_config: &mut PlayConfig) -> Result<(), u8> {
        let mut lines: Vec<String> = Vec::new();

        match grab_trimmed_file_lines(config_file_name, &mut lines) {
            Ok(()) =>
                {
                    // return error if not enough lines to generate the script
                    if lines.len() <= CHARACTER_CONFIG_LINE { return Err(FAILED_TO_GENERATE_SCRIPT); }

                    // add the config lines to the play configuration data structure
                    Ok(for line in lines {
                        self.add_config(&line, play_config);
                    })
                }
            Err(..) => Err(FAILED_TO_GENERATE_SCRIPT)
        }
    }

    /// call methods to prepare the scene
    pub fn prepare(&mut self, config_file_name: &String) -> Result<(), u8> {
        let mut play_config: PlayConfig = vec![];

        match self.read_config(config_file_name, &mut play_config) {
            Ok(..) => match self.process_config(play_config) {
                Ok(..) => {
                    //  after all Player structs have been added, sort them by lines
                    self.players.sort();
                    Ok(())
                },
                Err(e) => Err(e)
            },
            Err(e) => Err(e)
        }
    }

    /// print out the lines for the scene
    pub fn recite(&mut self) {
        let mut cur_line: usize = 0;
        let mut line_exists = true;
        let mut lines_spoken: usize;
        let mut last_speaker = String::new();

        while line_exists {
            line_exists = false;
            lines_spoken = 0;

            for player in &mut self.players {

                if let Some(line_num) = player.next_line() {
                    line_exists = true;

                    if line_num == cur_line {
                        player.speak(&mut last_speaker);
                        lines_spoken += 1;
                    }
                }
            }
            if WHINGE_MODE.load(Ordering::SeqCst) {
                if lines_spoken == 1 {

                } else {
                    if line_exists {
                        if lines_spoken == 0 {
                            eprintln!("ERROR: Missing line {}", cur_line);
                        } else {
                            eprintln!("ERROR: Duplicate line on line {}", cur_line);
                        }
                    }
                }
            }
            cur_line += 1;
        }

    }
}

