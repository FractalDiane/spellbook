// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

use crate::variant::Variant;
use crate::cauldron::*;
use crate::page::*;
use crate::constants::*;
use crate::sb_panic;

use rand::prelude::*;
use std::fs::OpenOptions;
use std::io::{self, Write};

pub struct Program {
	pages: [Page; 4],
	cauldron: Cauldron,

	spell_line_stack: Vec<(usize, usize)>,

	current_page: usize,
	turned_to_any_page: bool,

	drawer: Vec<Page>,
	floor: Option<Page>,
	memory: Option<Variant>,

	custom_signature: String,
	use_custom_signature: bool,

	pub line_internal: usize,
	pub line_number: usize,
	pub exit: bool,

	pub debug_mode: bool,
}

impl Program {
	pub fn new(debug_mode: bool) -> Self {
		let pages = [Page::new(PageType::Boolean), Page::new(PageType::Integer),
					Page::new(PageType::Float),Page::new(PageType::Str)];
		
		Self{
			pages,
			cauldron: Cauldron::new(),

			spell_line_stack: vec![],

			current_page: 0,
			turned_to_any_page: false,

			drawer: Vec::<Page>::new(),
			floor: None,
			memory: None,

			custom_signature: String::new(),
			use_custom_signature: false,

			line_internal: 0,
			line_number: 1,
			exit: false,

			debug_mode,
		}
	}

	pub fn change_line_by(&mut self, by: &Variant) {
		match by {
			Variant::Integer(int) => {
				if *int >= 0 {
					self.line_internal += *int as usize;
				} else {
					self.line_internal -= -*int as usize;
				}
			},
			_ => {
				sb_panic!(self.line_number);
			}
		}
	}

	pub fn is_totally_empty(&self) -> bool {
		!self.pages.iter().any(|p| p.has_any_contents())
	}

	fn can_write_to_page(&self, page_index: usize, name: &String) -> bool {
		self.turned_to_any_page
		&& !self.pages[page_index].entry_names.contains(name)
		&& !self.pages[page_index].is_full()
	}

	pub fn turn_to_page(&mut self, page_index: usize) {
		self.current_page = page_index;
		self.turned_to_any_page = true;
	}

	pub fn set_signature(&mut self, signature: String) {
		self.custom_signature = signature;
		self.use_custom_signature = true;
	}

	pub fn write_literal_value(&mut self, name: String, value: Option<Variant>) {
		if !self.can_write_to_page(self.current_page, &name) {
			sb_panic!(self.line_number);
		}

		if !self.pages[self.current_page].write_value(name, value, false, 0) {
			sb_panic!(self.line_number);
		}
	}

	pub fn write_memory_value(&mut self, name: String) {
		if !self.can_write_to_page(self.current_page, &name) {
			sb_panic!(self.line_number);
		}

		let mut rng = thread_rng();
		let new_value = match &self.memory {
			Some(val) => match val {
				Variant::Integer(int) => {
					let as_str = int.to_string();
					let modified = as_str.chars().enumerate().map(|(i, digit)| {
						let d = digit as u8;
						if rng.gen_bool(self.forget_chance(i)) {
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
						if d != b'.' && rng.gen_bool(self.forget_chance(i)) {
							(rng.gen_range(0..=9) + b'0') as char
						} else {
							d as char
						}
					}).fold(String::with_capacity(10), |mut st, c| { st.push(c); st });

					Some(Variant::Float(modified.parse::<f64>().unwrap()))
				},
				Variant::Str(ref string) => {
					let mut modified = string.clone();
					for word in string.split_whitespace().enumerate() {
						if rng.gen_bool(self.forget_chance(word.0)) {
							if rng.gen_bool(0.25) {
								let index = modified.find(word.1).unwrap();
								modified = modified.replace(word.1, "");
								modified.remove(index - 1);
							} else {
								modified = modified.replace(word.1, "something");
							}
						}
					}

					Some(Variant::Str(modified))
				},
				_ => Some(val.clone()),
			},
			None => {
				sb_panic!(self.line_number);
			},
		};

		if !self.pages[self.current_page].write_value(name, new_value, false, 0) {
			sb_panic!(self.line_number);
		}
	}

	pub fn get_value_by_index(&self, index: usize) -> Option<Variant> {
		self.pages[self.current_page].values[index].clone()
	}

	pub fn try_get_value(&self, name: &String) -> Option<Variant> {
		self.pages[self.current_page].read_value_by_name(&name)
	}

	pub fn cast_cauldron_spell(&mut self, spell: &CauldronSpell) {
		match self.cauldron.cast_spell(spell) {
			Some(CauldronSpellResult::DoNothing) => {},
			Some(CauldronSpellResult::NoCharge) => {
				return;
			},
			Some(CauldronSpellResult::SkipLine(charge)) => {
				self.spell_line_stack.push((self.line_internal, self.cauldron.get_amplifier()));
				self.cauldron.increase_charge(false, 0);
				self.line_internal += charge;
				return;
			},
			Some(CauldronSpellResult::JumpBack(charge)) => {
				self.line_internal = match self.spell_line_stack.get(charge) {
					Some(line) => line.0 - 1,
					None => {
						sb_panic!(self.line_number);
					},
				};

				for _ in charge..self.spell_line_stack.len() - 1 {
					self.cauldron.decrease_charge(true, self.spell_line_stack.last().unwrap_or_else(||
						sb_panic!(self.line_number)
					).1);
					self.spell_line_stack.remove(charge + 1);
				}
				
				self.cauldron.reset_amplifier();
				return;
			},
			None => {
				sb_panic!(self.line_number);
			},
		}

		self.spell_line_stack.push((self.line_internal, self.cauldron.get_amplifier()));
		self.cauldron.increase_charge(false, 0);
	}

	pub fn knock_over_cauldron(&mut self) {
		self.floor = self.cauldron.knock_over();
	}

	pub fn sign_page(&mut self, with: String) {
		if self.pages[self.current_page].changed_signature {
			sb_panic!(self.line_number);
		}

		self.pages[self.current_page].signature = with;
		self.pages[self.current_page].changed_signature = true;
	}

	pub fn publish(&self, not_console: bool, target: String) {
		let mut output = String::with_capacity(100);
		for p in 0..4 {
			for v in 0..3 {
				let concat = match &self.pages[p].values[v] {
					Some(val) => val.print(),
					None => String::new(),
				};

				output.push_str(&concat);
				if !concat.is_empty() {
					output.push_str(&self.pages[p].signature);
				}
			}
		}

		let signature = if self.use_custom_signature { self.custom_signature.clone() } else {
			if self.pages[PageType::Str as usize].has_any_contents() || self.is_totally_empty() {
				DEFAULT_WRAPUP.into()
			} else {
				DEFAULT_WRAPUP_QED.into()
			}
		};

		if !not_console {
			print!("{}{}", output, signature);
			io::stdout().flush().unwrap_or_else(|_| sb_panic!(self.line_number));
		} else {
			let mut outfile = OpenOptions::new()
				.create(true)
				.write(true)
				.append(true)
				.open(&target)
				.unwrap_or_else(|_| sb_panic!(self.line_number));

			write!(outfile, "{}{}", output, signature).unwrap_or_else(|_| sb_panic!(self.line_number));
		}
	}

	pub fn tear_out_page(&mut self, put_in_drawer: bool, put_in_cauldron: bool) {
		if !self.turned_to_any_page {
			sb_panic!(self.line_number);
		}

		if put_in_drawer {
			self.drawer.push(self.pages[self.current_page].clone());
		}

		if put_in_cauldron {
			if !self.cauldron.add_page(&self.pages[self.current_page]) {
				sb_panic!(self.line_number);
			}
		}
		
		self.pages[self.current_page].clear_page();
	}

	pub fn put_back_page(&mut self, from_drawer: bool) {
		if !self.turned_to_any_page {
			sb_panic!(self.line_number);
		}

		let page = if from_drawer {
			match self.drawer.pop() {
				Some(ref pg) => {
					pg.clone()
				},
				None => {
					sb_panic!(self.line_number);
				},
			}
		} else {
			match &self.floor {
				Some(pg) => pg.clone(),
				None => {
					sb_panic!(self.line_number);
				}
			}
		};
		
		for i in 0..3 {
			if !self.pages[self.current_page].write_value(page.entry_names[i].clone(), page.values[i].clone(), true, i) {
				sb_panic!(self.line_number);
			}
		}
	}

	pub fn memorize_value(&mut self, value: Option<Variant>) {
		self.memory = value;
	}

	fn forget_chance(&self, x: usize) -> f64 {
		if x < 5 {
			0.0
		} else {
			1.0 - (1.0 / (((x - 4) as f64).powf(0.3)))
		}
	}
}
