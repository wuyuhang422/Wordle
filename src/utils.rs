use std::{io::{self, Write, Read}, collections::HashMap};
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