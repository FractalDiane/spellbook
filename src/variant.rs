#[derive(Debug, PartialEq, Clone)]
pub enum Variant {
	Boolean(bool),
	Integer(i64),
	Float(f64),
	Str(String),
	Routine(fn(Variant) -> ()),
}

impl Variant {
	pub fn print(&self) -> String {
		match self {
			Variant::Boolean(b) => b.to_string(),
			Variant::Integer(i) => i.to_string(),
			Variant::Float(f) => f.to_string(),
			Variant::Str(s) => s.to_string(),
			Variant::Routine(_) => "BLAH BLAH BLAH".into(),
		}
	}

	pub fn to_bool(&self) -> bool {
		match self {
			Variant::Boolean(b) => *b,
			Variant::Integer(i) => *i != 0,
			Variant::Float(f) => *f != 0.0,
			Variant::Str(s) => !s.is_empty(),
			Variant::Routine(_) => {
				panic!();
			}
		}
	}

	pub fn to_int(&self) -> i64 {
		match self {
			Variant::Boolean(b) => *b as i64,
			Variant::Integer(i) => *i,
			Variant::Float(f) => *f as i64,
			Variant::Str(s) => s.parse::<i64>().unwrap(),
			Variant::Routine(_) => {
				panic!();
			}
		}
	}

	pub fn to_float(&self) -> f64 {
		match self {
			Variant::Boolean(b) => *b as i64 as f64,
			Variant::Integer(i) => *i as f64,
			Variant::Float(f) => *f,
			Variant::Str(s) => s.parse::<f64>().unwrap(),
			Variant::Routine(_) => {
				panic!();
			}
		}
	}

	pub fn to_string(&self) -> String {
		match self {
			Variant::Boolean(b) => b.to_string(),
			Variant::Integer(i) => i.to_string(),
			Variant::Float(f) => f.to_string(),
			Variant::Str(s) => s.to_string(),
			Variant::Routine(_) => {
				panic!();
			}
		}
	}
}
