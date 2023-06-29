use console;
use std::io::{self, Write};
use clap::Parser;
use rand::Rng;

pub mod interact_model;
mod builtin_words;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args{
    #[arg(short, long, default_value_t = String::new())]
    word: String,

    #[arg(short, long, default_value_t = false)]
    random: bool,

    #[arg(short='D', long, default_value_t = false)]
    difficult: bool,
}

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut rng = rand::thread_rng();

    let is_tty = atty::is(atty::Stream::Stdout);
    if is_tty {
        utils::clear_command_screen();
        print!("{}", console::style("Your name: ").bold().red());
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        println!("Welcome to Wordle, {}!", line.trim());
    }

    let mut answer = String::new();
    if args.random {
        // random 
        assert_eq!(args.word.len(), 0);
        let len = builtin_words::FINAL.len();
        answer = String::from(builtin_words::FINAL[rng.gen_range(0..len)]);
    }
    else if args.word.len() > 0 {
        // read from args
        answer = args.word;
    }
    else{
        if is_tty {
            println!("Please setting the answer:");
        }
        // read from stdin
        io::stdin().read_line(&mut answer)?;
    }

    interact_model::game_runner(&answer, is_tty, args.difficult)
}
