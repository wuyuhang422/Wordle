use serde::{Deserialize, Serialize};
use crate::utils::Stats;
use std::{collections::HashMap};

#[derive(Serialize, Deserialize)]
pub struct Gamejson{
	#[serde(default = "default_total_rounds")]
	total_rounds: u32, 
	#[serde(default = "default_games")]
	games: Vec<Games>,
}

fn default_total_rounds() -> u32{0}
fn default_games() -> Vec<Games> {Vec::new()}

#[derive(Serialize, Deserialize)]
pub struct Games {
	answer: String, 
	guesses: Vec<String>,
}

impl Games {
	pub fn new() -> Games {
		Games { answer: String::new(), guesses: Vec::new() }
	}

	pub fn set_answer(&mut self, user_input: String) {
		self.answer = user_input;
	}

	pub fn add_guess(&mut self, user_input: String){
		self.guesses.push(user_input);
	}

	pub fn check_result(&self) -> bool {
		// 1: win, 0: lose
		self.answer == *self.guesses.last().unwrap()
	}

	pub fn guesses_number(&self) -> i32 {
		self.guesses.len() as i32
	}

	pub fn get_guesses(&self) -> Vec<String> {
		self.guesses.clone()
	}
}

impl Gamejson {
	pub fn new() -> Gamejson {
		Gamejson { total_rounds: 0, games: Vec::new() }
	}

	pub fn get_total(&self) -> i32 {
		self.total_rounds as i32
	}

	pub fn add_games(&mut self, game: Games){
		self.games.push(game);
		self.total_rounds += 1;
	}

	pub fn to_stats(&self) -> Stats {
		let mut map: HashMap<String, i32> = HashMap::new();
		let mut win_count = 0;
		let mut attempts_count = 0;
		for game in &self.games {
			if game.check_result() {
				win_count += 1;
				attempts_count += game.guesses_number();
			}
			let tmp = game.get_guesses();
			for val in tmp {
				let count = map.entry(val).or_insert(0);
				*count += 1;
			}
		}
		Stats::new(Some(win_count), Some(self.get_total()),
		 Some(attempts_count), Some(0), Some(map))
	}
}

pub fn read_json(raw_json: &str) -> Gamejson {
	let parsed: Gamejson = serde_json::from_str(raw_json).unwrap();
	parsed
}