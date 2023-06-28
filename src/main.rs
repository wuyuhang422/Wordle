use console;
use std::io::{self, Write, Read};
use clap::Parser;
use termion;

pub mod interact_model;
mod builtin_words;
mod utils;

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);

    if is_tty {
        utils::clear_command_screen();
        println!(
            "I am in a tty. Please print {}!",
            console::style("colorful characters").bold().blink().blue()
        );
    } else {
        // println!("I am not in a tty. Please print according to test requirements!");
    }

    if is_tty {
        print!("{}", console::style("Your name: ").bold().red());
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        println!("Welcome to Wordle, {}!", line.trim());
        println!("Please setting the answer:");
    }

    // read in answer and play games!
    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;

    let mut gameinfo = interact_model::GameInfo::new(answer.trim());
    if is_tty {
        println!("Try to Make a Guess!");
    }
    
    while gameinfo.game_is_running() {
        let mut user_guess = String::new();
        io::stdin().read_line(&mut user_guess)?;
        let result = gameinfo.make_guess(user_guess.trim());
        match result{
            Ok(()) => gameinfo.print_process(is_tty), 
            Err(()) => {
                // print!("ERRERR!");
                // print!("{}",gameinfo.game_is_running());
                // assert!(false);
                if is_tty{
                    println!("Wrong Input! Please re-entering a new word!");
                }
                else{
                    println!("INVALID");
                }
            }
        }
    }

    Ok(())
}
