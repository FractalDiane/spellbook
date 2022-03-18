use crate::variant::Variant;

#[derive(Clone)]
pub enum PageType {
	Boolean,
	Integer,
	Float,
	Str,
	Routine,
}

struct Entry {
	boolean: bool,
	integer: i64,
	float: f64,
	string: String,
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
			entry_names: [String::new(), String::new(), String::new()],
			values: [None, None, None],
			page_type,
			write_index: 0,
		}
	}

	pub fn read_value(&self, index: usize) -> Option<&Variant> {
		self.values[index].as_ref()
	}

	pub fn read_value_by_name(&self, name: &String) -> Option<&Variant> {
		for i in 0..3 {
			if &self.entry_names[i] == name {
				return self.values[i].as_ref();
			}
		}

		None
	}

	pub fn write_value(&mut self, value: Option<Variant>, index_override: bool, index: usize) {
		let index = if index_override { index } else { self.write_index };
		match value {
			Some(val) => {
				let value_to_write = match self.page_type {
					PageType::Boolean => {
						Variant::Boolean(val.to_bool())
					},
					PageType::Integer => {
						Variant::Integer(val.to_int())
					},
					PageType::Float => {
						Variant::Float(val.to_float())
					},
					PageType::Str => {
						Variant::Str(val.to_string())
					},
					PageType::Routine => {
						Variant::Routine(|v| {})
					},
				};

				self.values[index] = Some(value_to_write);
				self.write_index += 1;
			},
			None => {
				self.values[index] = None;
			},
		}
	}

	pub fn clear_page(&mut self) {
		self.values = [None, None, None];
	}
}