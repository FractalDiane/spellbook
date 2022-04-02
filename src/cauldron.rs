// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

use crate::variant::Variant;
use crate::page::Page;
use std::io;
use std::slice::Iter;

#[derive(Debug, Clone)]
pub enum CauldronSpell {
	Entwine,
	Coadjuvancy,
	Stoachastize,
	Redesign,
}

impl CauldronSpell {
	// FIXME: This is bad do this a better way please
	fn iter() -> Iter<'static, Self> {
		const ALL_SPELLS: [CauldronSpell; 4] = [
			CauldronSpell::Entwine,
			CauldronSpell::Coadjuvancy,
			CauldronSpell::Stoachastize,
			CauldronSpell::Redesign,
		];

		ALL_SPELLS.iter()
	}
	pub fn from_string(string: &String) -> Option<Self> {
		for var in CauldronSpell::iter() {
			if *string == format!("{:?}", var) {
				return Some(var.clone());
			}
		}

		None
	}
}

pub struct Cauldron {
	page: Option<Page>,
}

impl Cauldron {
	pub fn new() -> Self {
		Self{
			page: None,
		}
	}

	pub fn add_page(&mut self, page: &Page) -> bool {
		match self.page {
			Some(ref mut my_page) => {
				for i in 0..3 {
					if page.values[i].is_some() {
						my_page.entry_names[i].push_str(&page.entry_names[i]);
						match my_page.values[i] {
							Some(ref current_val) => {
								match current_val.add(page.values[i].as_ref().unwrap().clone()) {
									Some(result) => {
										my_page.values[i] = Some(result);
									},
									None => {
										return false;
									},
								}
							},
							None => {
								my_page.values[i] = page.values[i].clone();
							},
						}
					}
				}
			},
			None => {
				self.page = Some(page.clone());
			},
		}

		true
	}

	pub fn knock_over(&mut self) -> Option<Page> {
		let page = self.page.clone();
		self.page = None;
		page
	}

	pub fn cast_spell(&mut self, spell: &CauldronSpell) -> bool {
		match spell {
			CauldronSpell::Coadjuvancy => {
				match self.page {
					Some(ref mut pg) => {
						if pg.is_full() {
							return false;
						}

						loop {
							let mut input = String::with_capacity(10);
							if io::stdin().read_line(&mut input).is_err() {
								return false;
							}

							let input_trimmed = input.trim_end(); 
				
							let var = if let Ok(b) = input_trimmed.parse::<bool>() {
								Variant::Boolean(b)
							} else if let Ok(i) = input_trimmed.parse::<i64>() {
								Variant::Integer(i)
							} else if let Ok(f) = input_trimmed.parse::<f64>() {
								Variant::Float(f)
							} else {
								Variant::Str(input_trimmed.to_string())
							};
				
							if pg.write_value(String::new(), Some(var), false, 0) {
								break;
							}
						}
					},
					None => {
						return false;
					},
				}
			},
			_ => {},
		}

		true
	}
}
