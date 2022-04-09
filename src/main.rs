// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

#![feature(iter_advance_by)]

mod cauldron;
mod constants;
mod errors;
mod page;
mod parser;
mod program;
mod variant;

use constants::ERROR_MESSAGES;
use program::Program;

use rand::prelude::*;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::panic;

fn main() {
	panic::set_hook(Box::new(|info| {
		let mut rng = thread_rng();
		let index: usize = rng.gen_range(0..10);
		let message = ERROR_MESSAGES[index];
		/*if let Some(error) = info.payload().downcast_ref::<SpellbookError>() {
			if error.line > 0 {
				eprintln!("\x1b[0;91mCatastrophe!\x1b[0m\n{}\n(Line {})", error.message, error.line);
			} else {
				eprintln!("\x1b[0;91mCatastrophe!\x1b[0m\n{}", error.message);
			}
		} else {
			eprintln!("\x1b[0;91mCatastrophe!\x1b[0m\nSomething utterly inexplicable happened.");
		}*/

		if let Some(line) = info.payload().downcast_ref::<usize>() {
			eprintln!("\x1b[0;91mCatastrophe!\x1b[0m\n{}\n(Line {})", message, line);
		} else {
			eprintln!("\x1b[0;91mCatastrophe!\x1b[0m\nSomething utterly inexplicable happened.");
		}
	}));

	let args = env::args().collect::<Vec<String>>();
	if args.len() < 2 {
		sb_panic!("Where's the instructions?!", 0);
	}

	let mut path = String::new();
	let mut debug_mode = false;
	for arg in &args[1..] {
		match arg.as_str() {
			"--trace" => {
				debug_mode = true;
			},
			_ => {
				path = arg.into();
			},
		}
	}

	if path.is_empty() {
		sb_panic!("Where's the instructions?!", 0);
	}

	let infile = File::open(&path).unwrap_or_else(|_| sb_panic!("You attempted to act on nonexistent instructions.", 0));

	let mut program = Program::new(debug_mode);
	let mut code = vec![];
	for line in io::BufReader::new(infile).lines().enumerate() {
		match line {
			(i, Ok(ln)) => {
				if !ln.trim().is_empty() {
					code.push((i + 1, ln));
				}
			},
			(_, Err(_)) => {},
		}
	}

	while program.line_internal < code.len() {
		program.line_number = code[program.line_internal].0;
		let tokenized = match parser::tokenize_line(code[program.line_internal].1.clone()) {
			Some(vec) => vec,
			None => {
				sb_panic!("Your instructions turned out to be gobbledegook.", program.line_number);
			},
		};

		if !tokenized.is_empty() {
			parser::execute_token_vector(&mut program, tokenized);
			if program.exit {
				return;
			}
		}

		if program.line_internal >= code.len() {
			sb_panic!("You magicked so hard that you flew off the instructions page.", program.line_number);
		}
		
		program.line_internal += 1;
	}
}
