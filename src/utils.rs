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
use std::{collections::HashSet, fs::{File, read_to_string}, io::{Read, self}};
pub struct WordDict {
	pub final_list: Vec<String>, 
	acceptable_list: Vec<String>,
	final_set: HashSet<String>, 
	acceptable_set: HashSet<String>
}

use crate::builtin_words;

impl WordDict {
	pub fn new() -> WordDict {
		WordDict { final_list: Vec::new(), acceptable_list: Vec::new(), final_set: HashSet::new(), acceptable_set: HashSet::new() }
	}

	pub fn vaild(&self, user_input: &str) -> bool {	
		// TODO: use hashtable to accelerate
		return self.acceptable_set.contains(&String::from(user_input));
	}

	pub fn build(&mut self, final_address: Option<String>, acceptable_address: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
		if acceptable_address.is_none() {
			for val in builtin_words::ACCEPTABLE {
				self.acceptable_list.push(String::from(*val).to_ascii_uppercase());
			}
		}
		else{
			let address = acceptable_address.unwrap();
			for line in read_to_string(address).unwrap().lines(){
				let word = line.trim().to_ascii_uppercase();
				self.acceptable_list.push(word);
			}
			
		}
		if final_address.is_none() {
			for val in builtin_words::FINAL {
				self.final_list.push(String::from(*val).to_ascii_uppercase());
			}
		}
		else{
			let address = final_address.unwrap();
			for line in read_to_string(address).unwrap().lines(){
				let word = line.trim().to_ascii_uppercase();
				self.final_list.push(word);
			}
		}
		self.final_set = self.final_list.clone().into_iter().collect::<HashSet<String>>();
		self.acceptable_set = self.acceptable_list.clone().into_iter().collect::<HashSet<String>>();
		if self.acceptable_list.len() != self.acceptable_set.len() {
			Err("acceptable words table have REPEATED words.".into())
		}
		else if self.final_list.len() != self.final_set.len() {
			Err("final words table have REPEATED words.".into())
		}
		else if !self.final_set.is_subset(&self.acceptable_set){
			Err("final set is not a subset of acceptable set".into())
		}
		else{
			Ok(())
		}
	}
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
    pub fn new(wins: Option<i32>, total: Option<i32>, attempts: Option<i32>, buffer: Option<i32>, guess_history: Option<HashMap<String, i32>>) -> Stats {
        Stats { wins: wins.unwrap_or(0), total: total.unwrap_or(0),
			 attempts: attempts.unwrap_or(0), buffer: buffer.unwrap_or(0), 
			 guess_history: guess_history.unwrap_or(HashMap::new()) }
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