extern crate lazy_static;
mod constants;
mod page;
mod parser;
mod variant;

use page::*;
use variant::Variant;
use constants::ERROR_MESSAGES;

use rand::prelude::*;

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::panic;

macro_rules! sb_panic {
	($line:expr) => {
		std::panic::panic_any($line);
	};
}

pub struct Program {
	pages: [Page; 5],

	current_page: usize,
	turned_to_any_page: bool,

	drawer: Vec<Page>,
	memory: Option<Variant>,
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
		}
	}

	pub fn turn_to_page(&mut self, page_index: usize) {
		self.current_page = page_index;
		self.turned_to_any_page = true;
	}

	pub fn write_value(&mut self, value: Option<Variant>) {
		self.pages[self.current_page].write_value(value, false, 0);
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
		self.memory = value;
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
	let mut line_num = 1usize;
	for line in io::BufReader::new(infile).lines() {
		let tokenized = match parser::tokenize_line(line.unwrap()) {
			Some(vec) => vec,
			None => {
				sb_panic!(line_num);
			},
		};

		if !parser::execute_token_vector(&mut program, tokenized) {
			sb_panic!(line_num);
		}

		line_num += 1;
	}
}
