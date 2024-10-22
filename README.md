# rust-lab2

## Modules 

Refactoring the code into modules was actually fairly simple for us.
We made the modifications detailed in the instructions (creating the lab2 directory
and moving declarations.rs and script_gen.rs into it).  We then made everything in 
declarations.rs public since that is what it is meant to be, and then in main.rs,
we went through and made sure we had no errors by creating use statements and 
changing the correct thing in script_gen.rs to be public.  We anticipate this to be
a continued effort as we progress through this lab.

## Structs

The detail of the structs was given in the assignment.
There are two main structs: Play and Player.

- Player: handles the individual character data like name, lines and current position.
- Play: handles the general script and contains a collection of players. More high level view.

I made the methods public and imported them as needed, everything else would remain private.