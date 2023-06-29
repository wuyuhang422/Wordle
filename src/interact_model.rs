// Process the whole game

use crate::utils::{self, Stats};
use std::io::{self};

const GUESS_CHANCE: i32 = 6;

pub enum GameStatus{
	Running,
	Success,
	Fail,
}

pub struct GameInfo{
	keyboard_status: [char; 26],
	guess_answer: Vec<char>,
	guess_history: Vec<(String,String)>,
	game_status: GameStatus,
	is_difficult: bool,
}

impl GameInfo{
	pub fn new(answer: &str, is_difficult: bool) -> GameInfo {
		GameInfo { keyboard_status: ['X'; 26], guess_answer: String::from(answer).chars().collect(), guess_history: Vec::new(), game_status: GameStatus::Running, is_difficult: is_difficult}
	}

	pub fn game_is_running(&self) -> bool {
		match self.game_status {
			GameStatus::Running => true,
			_ => false,
		}
	}

	fn change_keyboard_status(&mut self, idx: usize, val: char) -> () {
		if utils::get_color_rank(val) > utils::get_color_rank(self.keyboard_status[idx]){
			self.keyboard_status[idx] = val;
		}
	}

	fn difficult_vaild(&mut self, user_input: &str) -> bool {
		match self.is_difficult {
			false => true,
			true => {
				if self.guess_history.len() == 0{
					true
				}
				else{
					let (last_input, last_color) = self.guess_history.last().unwrap();
					let tmp = last_input.chars().zip(last_color.chars());
					let _tmp = String::from(user_input);
					let tmp = tmp.zip(_tmp.chars());
					let mut last_cnt = [0; 26];
					let mut now_cnt = [0; 26];
					for ((last_char, last_col), now_char) in tmp {
						if last_col == 'G' && last_char != now_char {
							return false;
						}
						if last_col == 'Y' {
							last_cnt[(last_char as usize) - ('a' as usize)] += 1;
						}
						if last_col != 'G' {
							now_cnt[(now_char as usize) - ('a' as usize)] += 1;
						}
					}
					for i in 0_usize..26_usize {
						if now_cnt[i] < last_cnt[i]{
							return false;
						}
					}
					true
				}
			}
		}
	}

	pub fn make_guess(&mut self, user_input: &str) -> Result<(),()> {
		/* We assume that guess_answer are vaild
		give in a user_guess, update the status for keyboard, screen, ...
		*/
		if !(utils::vaild(user_input) && self.difficult_vaild(user_input)){
			return Err(())
		}
		let user_input = String::from(user_input);
		let t:Vec<char> = user_input.chars().collect();
		let mut guess_status = String::new();
		let mut flag = true;
		let mut cnt = [0; 26];
		for ch in &self.guess_answer{
			cnt[(*ch as usize) - ('a' as usize)] += 1;
		}
		/*  fix: you should firstly consider green character.
			example:
			answer = START
			user_input = ABAND
		*/
		for i in 0..t.len(){
			let id = (t[i] as usize) - ('a' as usize);
			if t[i] == self.guess_answer[i]{
				cnt[id] -= 1;
			}
		}
		for i in 0..t.len(){
			// println!("{}, {}",t[i] as usize, 'a' as usize);
			let id = (t[i] as usize) - ('a' as usize);
			let mut res: char = 'X';
			if t[i] == self.guess_answer[i]{
				res = 'G';
			}
			else if cnt[id] > 0{
				res = 'Y';
				cnt[id] -= 1;
			}
			else{
				res = 'R';
			}
			flag &= res == 'G';
			self.change_keyboard_status(id, res);
			guess_status.push(res);
		}
		self.guess_history.push((user_input, guess_status));
		if flag{
			self.game_status = GameStatus::Success;
		}
		else if self.guess_history.len() == (GUESS_CHANCE as usize){
			self.game_status = GameStatus::Fail;
		}
		return Ok(());
	}

	pub fn print_process(&self, is_tty: bool) -> (){
		/*
		If is_tty is true, then will print colorful guess history and keyboard status
		Else they will print debug message.
			*/
		if is_tty {
			// TODO: print keyboard as its own order
			utils::clear_command_screen();
			println!("Guess History:");
			for (user_input, guess_status) in &self.guess_history{
				let tmp = user_input.chars().zip(guess_status.chars());
				for (ch, col) in tmp{
					utils::print_with_color(ch.to_ascii_uppercase(), col);
				}
				print!("\n");
			}
			for i in 0_usize..25_usize{
				let now_char = (('A' as u8) + (i as u8)) as char;
				utils::print_with_color(now_char.to_ascii_uppercase(), self.keyboard_status[i]);
			}
			print!("\n");
		}
		else{
			let (_, guess_status) = &self.guess_history[self.guess_history.len()-1];
			for ch in guess_status.chars(){
				print!("{}",ch.to_ascii_uppercase());
			}
			print!(" ");
			for ch in self.keyboard_status{
				print!("{}",ch.to_ascii_uppercase());
			}
			print!("\n");
		}
		match self.game_status{
			GameStatus::Running => {
				if is_tty {
					println!("You have {} more chance, have fun!", GUESS_CHANCE - self.guess_history.len() as i32);
					println!("Try to Make a Guess!");
				}
			},
			GameStatus::Success => {
				if is_tty {
					println!("Good Job, You Win!");
				}
				else{
					println!("CORRECT {}", self.guess_history.len());
				}
			},
			GameStatus::Fail => {
				if is_tty {
					println!("Oh, You've used up all your chances!");
					print!("The correct answer is: ");
					for ch in &self.guess_answer{
						print!("{}", ch.to_ascii_uppercase());
					}
					print!("\n");
				}
				else{
					print!("FAILED ");
					for ch in &self.guess_answer{
						print!("{}", ch.to_ascii_uppercase());
					}
					print!("\n");
				}
			},
		}
	}
}

pub fn game_runner(answer: &str, is_tty: bool, is_difficult: bool, stats: &mut Stats) -> Option<bool> {
	let mut gameinfo = crate::interact_model::GameInfo::new(answer.trim(), is_difficult);
	if is_tty {
		println!("Try to Make a Guess!");
	}
	
	while gameinfo.game_is_running() {
		let mut user_guess = String::new();
		let tmp = io::stdin().read_line(&mut user_guess);
		assert!(tmp.is_ok());
		let result = gameinfo.make_guess(user_guess.trim());
		match result{
			Ok(()) => {
				gameinfo.print_process(is_tty);
				stats.add_guess(String::from(user_guess.trim()));
			}
			Err(()) => {
				if is_tty{
					println!("Wrong Input! Please re-entering a new VALID word!");
				}
				else{
					println!("INVALID");
				}
			}
		}
	}

	match gameinfo.game_status {
		GameStatus::Running => None, 
		GameStatus::Success => Some(true), 
		GameStatus::Fail => Some(false),
	}
}