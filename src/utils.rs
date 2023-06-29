pub fn get_color_rank(val: char) -> Option<i32> {
	match val {
		'G' => Some(3), 
		'Y' => Some(2),
		'R' => Some(1),
		'X' => Some(0),
		_ => None,
	}
}

pub fn clear_command_screen() -> (){
	print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1))
}

use console;

pub fn print_with_color(ch: char, color: char) -> (){
	match color{
		'G' => print!("{}", console::style(format!("{}", ch)).bold().green()),
		'R' => print!("{}", console::style(format!("{}", ch)).bold().red()),
		'Y' => print!("{}", console::style(format!("{}", ch)).bold().yellow()),
		'X' => print!("{}", console::style(format!("{}", ch)).bold().white()),
		_ => assert!(false)
	}
}

use crate::builtin_words;

pub fn vaild(user_input: &str) -> bool {
	// TODO: use hashtable to accelerate
	for val in builtin_words::ACCEPTABLE {
		if **val == *user_input{
			return true;
		}
	}
	return false;
}
use std::collections::HashMap;
pub struct Stats{
    wins: i32,
    total: i32,
    attempts: i32,
    buffer: i32,
    guess_history: HashMap<String, i32>,
}

impl Stats{
    pub fn new() -> Stats {
        Stats { wins: 0, total: 0, attempts: 0, buffer: 0, guess_history: HashMap::new() }
    }

    pub fn add_game(&mut self, result: bool){
        // result: if win?
        self.total += 1;
        if result {
            self.wins += 1;
            self.attempts += self.buffer;
        }
        self.buffer = 0;
    }

    pub fn add_guess(&mut self, user_input: String){
        let count = self.guess_history.entry(user_input).or_insert(0);
        *count += 1;
        self.buffer += 1;
    }

    fn att_rate(&self) -> f32 {
        match self.wins {
            0 => 0.0,
            _ => (self.attempts as f32) / (self.wins as f32),
        }
    }

    fn get_top5_words(&self) -> Vec<(i32, String)> {
        let mut result: Vec<(i32, String)> = Vec::new();
        for (key, value) in &self.guess_history {
            result.push((-*value, key.clone()));
        }
        result.sort();
        while result.len() > 5 {
            result.pop();
        }
        result
    }

    pub fn print_result(&self, is_tty: bool){
        let vec = self.get_top5_words();
        if is_tty {
            println!("Your Game Performance:");
            println!("Win: {}, Lose: {}, Average attempts in win game: {:.2}", self.wins, self.total - self.wins, self.att_rate());
            println!("Your TOP5 favorite words:");
            for i in 0..vec.len() {
                let (count, user_input) = &vec[i];
                println!("Rank {}: {}, used {} times.", i+1, user_input.to_ascii_uppercase(), -count);
            }
        }
        else{
            println!("{} {} {:.2}", self.wins, self.total - self.wins, self.att_rate());
            for i in 0..vec.len() {
				let (count, user_input) = &vec[i];
				if i > 0{
					print!(" ");
				}
                print!("{} {}", user_input.to_ascii_uppercase(), -count);
            }
            print!("\n");
        }
    }
}