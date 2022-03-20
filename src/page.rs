// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

use crate::variant::Variant;

#[derive(Clone)]
pub enum PageType {
	Boolean,
	Integer,
	Float,
	Str,
	Routine,
}

#[derive(Clone)]
pub struct Page {
	pub entry_names: [String; 3],
	pub values: [Option<Variant>; 3],

	page_type: PageType,
	write_index: usize,
}

impl Page {
	pub fn new(page_type: PageType) -> Self {
		Self{
			entry_names: [String::with_capacity(10), String::with_capacity(10), String::with_capacity(10)],
			values: [None, None, None],
			page_type,
			write_index: 0,
		}
	}

	pub fn read_value_by_name(&self, name: &String) -> Option<Variant> {
		for i in 0..3 {
			if &self.entry_names[i] == name {
				return self.values[i].clone();
			}
		}

		None
	}

	pub fn write_value(&mut self, name: String, value: Option<Variant>, index_override: bool, index: usize) -> bool {
		let ind = if index_override { index } else { self.write_index };
		self.entry_names[ind] = name;
		match value {
			Some(val) => {
				let value_to_write = match self.page_type {
					PageType::Boolean => {
						Variant::Boolean(val.to_bool())
					},
					PageType::Integer => {
						if let Some(i) = val.to_int() {
							Variant::Integer(i)
						} else {
							return false;
						}
					},
					PageType::Float => {
						if let Some(f) = val.to_float() {
							Variant::Float(f)
						} else {
							return false;
						}
					},
					PageType::Str => {
						Variant::Str(val.to_string())
					},
					PageType::Routine => {
						Variant::Routine(|_| {})
					},
				};

				self.values[ind] = Some(value_to_write);
				self.write_index += 1;

				true
			},
			None => {
				self.entry_names[ind] = String::new();
				self.values[ind] = None;

				true
			},
		}
	}

	pub fn has_any_contents(&self) -> bool {
		self.values.iter().any(|v| v.is_some())
	}

	pub fn is_full(&self) -> bool {
		!self.values.iter().any(|v| v.is_none())
	}

	pub fn clear_page(&mut self) {
		self.values = [None, None, None];
		for name in &mut self.entry_names {
			name.clear();
		}

		self.write_index = 0;
	}
}
