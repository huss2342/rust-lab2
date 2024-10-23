/// ADD FILE HEADER
///
///

use crate::lab2::declarations::{CharName, FAILED_TO_GENERATE_SCRIPT, WHINGE_MODE};
use crate::lab2::player::Player;
use crate::lab2::script_gen::{grab_trimmed_file_lines, CharacterTextFile,
                              CHARACTER_FILE_CONFIG_LINE_INDEX, CHARACTER_NAME_CONFIG_LINE_INDEX,
                              CONFIG_LINE_TOKENS, TITLE_LINE, CHARACTER_CONFIG_LINE};
use std::sync::atomic::Ordering;

type PlayConfig = Vec<(CharName, CharacterTextFile)>;

pub struct SceneFragment {
    title: String,
    players: Vec<Player>,
}

impl SceneFragment {

    /// TODO Add Documentation
    pub fn new(title: String) -> SceneFragment {
        SceneFragment {
            title,
            players: Vec::new(),
        }
    }

    ///
    /// Print a message for every player that needs to enter for the next scene.
    ///
    /// # Parameters
    ///
    /// - `self`: A reference to self
    /// - `other`: A reference to another instance of the struct SceneFragment
    ///
    fn enter(&self, other: &SceneFragment) {
        for player in &other.players {
            // if a player in the other scene is not in this scene
            if !self.players.contains(&player) {
                println!("[Enter {}.]", player.name);
            }
        }
    }

    ///
    /// Print a message for every player in this scene stating that they are entering.
    ///
    /// # Parameters
    ///
    /// - `self`: A reference to self
    ///
    fn enter_all(&self) {
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
    /// - `other`: A reference to another instance of the struct SceneFragment
    ///
    fn exit(&self, other: &SceneFragment) {
        for player in self.players.iter().rev() {
            // if a player in this scene is not in the other scene
            if !other.players.contains(&player) {
                println!("[Enter {}.]", player.name);
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
    fn exit_all(&self) {
        for player in self.players.iter().rev() {
            println!("[Enter {}.]", player.name);
        }
    }


    /// TODO Add Documentation
    fn process_config(&mut self, play_config: PlayConfig) -> Result<(), u8> { // I changed this to not be pub, hopefully that is fine
        for config in play_config {
            match config {
                (char_name, part_file_name) => {
                    let mut player = Player::new(&char_name);
                    if let Err(e) = player.prepare(&part_file_name) {
                        eprintln!("[X] ERROR: Failed to generate script for character {}.",
                                  char_name);
                        return Err(e);  // FIXME this was a quick fix and possibly could be written better -Nick
                    }
                    self.players.push(player);
                }
            }
        }
        self.players.sort();
        Ok(())
    }

    /// TODO Add Documentation
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

    /// TODO Add Documentation
    fn read_config(&self, config_file_name: &String, play_title: &mut String,
                   play_config: &mut PlayConfig) -> Result<(), u8> {
        let mut lines: Vec<String> = Vec::new();

        match grab_trimmed_file_lines(config_file_name, &mut lines) {
            Ok(()) =>
                {
                    // return error if not enough lines to generate the script
                    if lines.len() <= CHARACTER_CONFIG_LINE { return Err(FAILED_TO_GENERATE_SCRIPT); }

                    *play_title = lines[TITLE_LINE].clone();

                    // adding the remaining lines to the play configuration data structure
                    Ok(for line in &lines[1..] {
                        self.add_config(line, play_config)
                    })
                }
            Err(..) => Err(FAILED_TO_GENERATE_SCRIPT)
        }
    }

    // was script_gen
    /// TODO Add Documentation
    pub fn prepare(&mut self, config_file_name: &String) -> Result<(), u8> {
        let mut play_config: PlayConfig = vec![];
        let mut play_title: String = String::new();

        match self.read_config(config_file_name, &mut play_title, &mut play_config) {
            Ok(..) => match self.process_config(play_config) {
                Ok(..) => Ok(()),
                Err(e) => Err(e)
            },
            Err(e) => Err(e)
        }
    }

    // TODO: really not sure about this one. Nick: yeah it's def not working based on errors
    // TODO: from part 12 and skipped over
    // Also modify the appropriate place in the SceneFragment struct's associated recite method that prints out the struct's title string, so that it only prints it if it is non-blank (has at least one non-whitespace token).
    pub fn recite(&mut self) {
        let mut last_speaker = String::new();
        let mut current_line = 0;

        for mut player in &mut self.players {
            if let Some(line_num) = player.next_line() {
                if WHINGE_MODE.load(Ordering::SeqCst) && current_line == 0 && line_num > 0 {
                    eprintln!("ERROR: Missing line 0");
                }

                if line_num == current_line {
                    player.speak(&mut last_speaker);
                }
            }
        }
        current_line += 1;
    }
}

