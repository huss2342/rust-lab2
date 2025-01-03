# rust-lab2

## Group Members
- Nick Cochran c.nick@wustl.edu
-  Hussein Aljorani a.hussein@wustl.edu
-  Becky Shofner r.a.shofner@wustl.edu

## Overview
Lab 2 builds on Lab 1 by modularizing our code to handle multiple scene files within
an act, improving flexibility and readability. To complete this lab, we worked together
to refactor Lab 1 into a modular structure. Each team member then focused on a specific 
section: Structs, Return Wrapper, or Scene Fragments, ensuring the code compiled and ran 
before handing the project off to the next partner. Finally, we collaborated on debugging 
and testing to ensure smooth functionality. 

## Modules 

Refactoring the code into modules was actually fairly simple for us.
We made the modifications detailed in the instructions (creating the lab2 directory
and moving declarations.rs and script_gen.rs into it).  We then made everything in 
declarations.rs public since that is what it is meant to be, and then in main.rs,
we went through and made sure we had no errors by creating use statements and 
changing the correct thing in script_gen.rs to be public.  We anticipate this to be
a continued effort as we progress through this lab.

## Structs

The code was refactored into two public structs:

### Player Struct
- Handles the individual character data (name, lines, current position)
- Handles line delivery and sorting
- Implements ordering traits for proper sequencing
- Prepares the player's own script lines and allows the player to speak the lines.

### Play Struct
- Manages script configuration and scene fragments
- Handles the scene transitions and character interactions
- Controls the general script playback
- Prepares and pushes the scene fragments and recites the play.

## Return Wrapper

The return wrapper was a pretty simple implementation at first.  I created the struct
and then implemented the report method as instructed.  There were a few complications though
as I first created my own trait called Termination and didn't realize until after trying to compile
that it needed to be using the trait from std::process.  Additionally, when implementing the new function
I first just had it taking in a u8 before realizing that I needed to take in a Result
that can be broken down in a match expression to initialize the ReturnWrapper struct.
Other than those two things, it worked as expected.

## Scene Fragments

The modification play.rs file was modified to have a StructConfig type, replacing PlayConfig, and replacing all 
variables in the Play struct with Fragments type vector of SceneFragment structs. Within the impl block, all 
functions were modified to replace old PlayConfig references and parameters, as well as changing names of 
constraints and variables. Based on the new types and instructions, some function modifications were minimal, 
like process_config(), while other functions were entirely overhauled, like recite(). Lastly, teh main file was
modified from config_file_name to script_file_name, as well as updating any constants and variables that pertained
to the old files. Design challenges were multiple mutable borrows in the recite function, which was solved by 
implementing an iterator over the fragments. I also needed to make the enter and exit functions public to be able 
for access in the recite function.

## Testing
Please refer to Testing_ouput.txt for outputs of various tests.
1. Run script file from terminal using incorrect file name
2. Mis-spelling of file name and/or "whinge"
3. Additional or less arguments in command line
4. Removal of "[scene]" in script file
5. Misspelling of tokens in config files
6. Removal and addition of number of tokens in config file
7. Addition and removal of whitespace in both config and text files
8. Removal of number token or script line in text file

## Usage
1. Unzip the project folder.
2. Write a script file with its config text files in the root of the project directory, or use the one provided.
3. Run the main script using the following command:
   ```
   cargo run <script_file_name> [whinge]
   ```
   Where:
- `<script_file_name>` is the name of your script file (required)
- `[whinge]` is an optional parameter to enable additional error output

  Example:
    ```
    cargo run partial_hamlet_act_ii_script.txt whinge
    ```
