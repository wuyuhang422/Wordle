use console;
use std::{io::{self, Write}};
use clap::Parser;
use rand::{Rng, seq::SliceRandom, SeedableRng};

pub mod interact_model;
mod utils;
mod builtin_words;
use utils::Stats;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args{
    #[arg(short, long)]
    word: Option<String>,

    #[arg(short, long, default_value_t = false)]
    random: bool,

    #[arg(short='D', long, default_value_t = false)]
    difficult: bool,

    #[arg(short='t', long, default_value_t = false)]
    stats: bool,

    #[arg(short, long)]
    day: Option<usize>,

    #[arg(short, long)]
    seed: Option<u64>, 

    #[arg(short, long)]
    final_set: Option<String>,

    #[arg(short, long)]
    acceptable_set: Option<String>
}

impl Args{
    fn have_conflicts(&self) -> bool {
        if self.random && self.word.is_some(){
            return true;
        }
        if self.word.is_some() {
            if self.day.is_some() || self.seed.is_some() {
                return true;
            }
        }
        false
    }
}

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.have_conflicts(){
        return Err("Args are conflict".into());
    }

    let mut word_dict = utils::WordDict::new();
    word_dict.build(args.final_set, args.acceptable_set)?;
    
    let mut rng = rand::rngs::StdRng::seed_from_u64(args.seed.unwrap_or(114514 as u64));

    let is_tty = atty::is(atty::Stream::Stdout);

    if is_tty {
        utils::clear_command_screen();
        print!("{}", console::style("Your name: ").bold().red());
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        println!("Welcome to Wordle, {}!", line.trim());
    }

    let len = word_dict.final_list.len();
    let mut select_order = vec![0_usize; len];
    for i in 0..len {
        select_order[i] = i;
    }
    select_order.shuffle(&mut rng);
    let mut idx = args.day.unwrap_or(1)-1;

    let mut stats = Stats::new();

    loop {
        let mut answer = String::new();
        if args.random {
            // random 
            assert!(idx < select_order.len());
            answer = word_dict.final_list[select_order[idx]].clone();
            idx += 1;
        }
        else if args.word.is_some() {
            // read from args
            answer = args.word.clone().unwrap();
        }
        else{
            if is_tty {
                println!("Please setting the answer:");
            }
            // read from stdin
            io::stdin().read_line(&mut answer)?;
        }

        let result = interact_model::game_runner(
            &answer, is_tty, args.difficult, &mut stats, &word_dict);
        stats.add_game(result.unwrap());
        if args.stats{
            stats.print_result(is_tty);
        }

        if args.word.is_some() {
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
