use console;
use std::io::{self, Write};
use clap::Parser;
use rand::{Rng, seq::SliceRandom};

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

    #[arg(short='t', long, default_value_t = false)]
    stats: bool,
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

    let len = builtin_words::FINAL.len();
    let mut select_order = vec![0_usize; len];
    for i in 0..len {
        select_order[i] = i;
    }
    select_order.shuffle(&mut rng);
    let mut idx = 0_usize;

    loop {
        let mut answer = String::new();
        if args.random {
            // random 
            assert_eq!(args.word.len(), 0);
            assert!(idx < select_order.len());
            answer = String::from(builtin_words::FINAL[select_order[idx]]);
            idx += 1;
        }
        else if args.word.len() > 0 {
            // read from args
            answer = args.word.clone();
        }
        else{
            if is_tty {
                println!("Please setting the answer:");
            }
            // read from stdin
            io::stdin().read_line(&mut answer)?;
        }

        interact_model::game_runner(&answer, is_tty, args.difficult)?;
        if args.word.len() > 0 {
            break;
        }
        
        if is_tty{
			println!("Do you want to start a new game?(Y/N)");
        }
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        if line.trim() != String::from("Y"){
            break;
        }
        if is_tty{
            utils::clear_command_screen();
        }
    }
    Ok(())
}
