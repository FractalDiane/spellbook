// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

#![feature(iter_advance_by)]

mod cauldron;
mod constants;
mod macros;
mod page;
mod parser;
mod program;
mod variant;

use program::Program;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::panic;

fn main() {
	/*panic::set_hook(Box::new(|info| {
		let mut rng = thread_rng();
		let index: usize = rng.gen_range(0..10);
		let message = ERROR_MESSAGES[index];
		if let Some(line) = info.payload().downcast_ref::<usize>() {
			eprintln!("\x1b[0;91mCatastrophe!\x1b[0m\n{}\n(Line {})", message, line);
		} else {
			eprintln!("\x1b[0;91mCatastrophe!\x1b[0m\n{}", message);
		}
	}));*/

	let args = env::args().collect::<Vec<String>>();
	if args.len() < 2 {
		panic!();
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
		panic!();
	}

	let infile = File::open(&path).unwrap();

	let mut program = Program::new(debug_mode);
	let mut code = vec![];
	for line in io::BufReader::new(infile).lines() {
		match line {
			Ok(ln) => {
				code.push(ln);
			},
			Err(_) => {},
		}
	}

	while program.line_number <= code.len() {
		if !code[program.line_number - 1].trim().is_empty() {
			let tokenized = match parser::tokenize_line(code[program.line_number - 1].clone()) {
				Some(vec) => vec,
				None => {
					sb_panic!(program.line_number);
				},
			};

			if !tokenized.is_empty() {
				parser::execute_token_vector(&mut program, tokenized);
				if program.exit {
					return;
				}
			}
		}

		if program.line_number < 1 || program.line_number > code.len() {
			sb_panic!(program.line_number);
		}
		
		program.line_number += 1;
	}
}
