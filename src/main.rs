// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

extern crate rand;
mod constants;
mod page;
mod parser;
mod variant;

use page::*;
use variant::Variant;
use constants::*;

use rand::prelude::*;

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::panic;

macro_rules! sb_panic {
	($line:expr) => {
		std::panic::panic_any($line);
	};
}

fn forget_chance(x: usize) -> f64 {
	if x < 5 {
		0.0
	} else {
		1.0 - (1.0 / (((x - 4) as f64).powf(0.3)))
	}
}

pub struct Program {
	pages: [Page; 5],

	current_page: usize,
	turned_to_any_page: bool,

	drawer: Vec<Page>,
	memory: Option<Variant>,

	line_number: usize,
}

impl Program {
	pub fn new() -> Self {
		let pages = [Page::new(PageType::Boolean), Page::new(PageType::Integer),
					Page::new(PageType::Float),Page::new(PageType::Str),
					Page::new(PageType::Routine)];
		
		Self{
			pages,
			current_page: 0,
			turned_to_any_page: false,
			drawer: Vec::<Page>::new(),
			memory: None,
			line_number: 1,
		}
	}

	pub fn turn_to_page(&mut self, page_index: usize) {
		self.current_page = page_index;
		self.turned_to_any_page = true;
	}

	pub fn write_literal_value(&mut self, value: Option<Variant>) {
		self.pages[self.current_page].write_value(value, false, 0);
	}

	pub fn publish(&self, target: String, override_end: bool, end: String) {
		let mut output = String::with_capacity(100);
		for p in 0..5 {
			for v in 0..3 {
				let concat = match &self.pages[p].values[v] {
					Some(val) => val.print(),
					None => String::new(),
				};

				output.push_str(&concat);
				if !concat.is_empty() {
					output.push('\n');
				}
			}
		}

		let wrapup = if override_end { end } else {
			if self.pages[PageType::Str as usize].has_any_contents() {
				DEFAULT_WRAPUP.into()
			} else {
				DEFAULT_WRAPUP_QED.into()
			}
		};

		if target == "console" {
			print!("{}{}", output, wrapup);
			io::stdout().flush().unwrap_or_else(|_| sb_panic!(self.line_number));
		} else {
			let mut outfile = OpenOptions::new()
				.write(true)
				.append(true)
				.open(&target)
				.unwrap_or_else(|_| sb_panic!(self.line_number));

			write!(outfile, "{}{}", output, wrapup).unwrap_or_else(|_| sb_panic!(self.line_number));
		}
	}

	pub fn tear_out_page(&mut self) {
		if !self.turned_to_any_page {
			panic!();
		}

		self.drawer.push(self.pages[self.current_page].clone());
		self.pages[self.current_page].clear_page();
	}

	pub fn put_back_page(&mut self) {
		let page = self.drawer.pop().unwrap();
		for i in 0..3 {
			self.pages[self.current_page].write_value(page.values[i].clone(), true, i);
		}
	}

	pub fn memorize_value(&mut self, value: Option<Variant>) {
		let result = match value {
			Some(ref val) => {
				let mut rng = thread_rng();
				match val {
					Variant::Str(string) => {
						let modified = string.split_whitespace().enumerate().map(|(i, word)| {
							if rng.gen_bool(forget_chance(i)) {
								"something".into()
							} else {
								word.to_string()
							}
						}).collect::<Vec<String>>();

						Some(Variant::Str(modified.join(" ")))
					},
					Variant::Integer(int) => {
						let as_str = int.to_string();
						let modified = as_str.chars().enumerate().map(|(i, digit)| {
							let d = digit as u8;
							if rng.gen_bool(forget_chance(i)) {
								(rng.gen_range(0..=9) + b'0') as char
							} else {
								d as char
							}
						}).fold(String::with_capacity(10), |mut st, c| { st.push(c); st });

						Some(Variant::Integer(modified.parse::<i64>().unwrap()))
					},
					Variant::Float(float) => {
						let as_str = float.to_string();
						let modified = as_str.chars().enumerate().map(|(i, digit)| {
							let d = digit as u8;
							if d != b'.' && rng.gen_bool(forget_chance(i)) {
								(rng.gen_range(0..=9) + b'0') as char
							} else {
								d as char
							}
						}).fold(String::with_capacity(10), |mut st, c| { st.push(c); st });

						Some(Variant::Float(modified.parse::<f64>().unwrap()))
					},
					_ => {
						value
					},
				}
			},
			None => None,
		};

		self.memory = result;
	}
}

fn main() {
	panic::set_hook(Box::new(|info| {
		let mut rng = thread_rng();
		let index: usize = rng.gen_range(0..10);
		let message = ERROR_MESSAGES[index];
		if let Some(line) = info.payload().downcast_ref::<usize>() {
			eprintln!("\x1b[0;91mCatastrophe!\x1b[0m\n{}\n(Line {})", message, line);
		} else {
			eprintln!("\x1b[0;91mCatastrophe!\x1b[0m\n{}", message);
		}
	}));

	let mut args = env::args();
	if args.len() != 2 {
		panic!();
	}

	args.next().unwrap();
	let path = args.next().unwrap();
	let infile = File::open(&path).unwrap();

	let mut program = Program::new();
	for line in io::BufReader::new(infile).lines() {
		match line {
			Ok(ln) => {
				if !ln.trim().is_empty() {
					let tokenized = match parser::tokenize_line(ln) {
						Some(vec) => vec,
						None => {
							sb_panic!(program.line_number);
						},
					};
			
					if !parser::execute_token_vector(&mut program, tokenized) {
						sb_panic!(program.line_number);
					}
				}
			},
			Err(_) => {},
		}

		program.line_number += 1;
	}
}
