// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Variant {
	Boolean(bool),
	Integer(i64),
	Float(f64),
	Str(String),
}

impl Variant {
	pub fn try_unwrap_bool(&self) -> Option<bool> {
		match self {
			Variant::Boolean(b) => Some(*b),
			_ => None,
		}
	}

	pub fn try_unwrap_int(&self) -> Option<i64> {
		match self {
			Variant::Integer(i) => Some(*i),
			_ => None,
		}
	}

	pub fn try_unwrap_float(&self) -> Option<f64> {
		match self {
			Variant::Float(f) => Some(*f),
			_ => None,
		}
	}

	pub fn try_unwrap_string(&self) -> Option<String> {
		match self {
			Variant::Str(s) => Some(s.clone()),
			_ => None,
		}
	}

	pub fn print(&self) -> String {
		match self {
			Variant::Boolean(b) => b.to_string(),
			Variant::Integer(i) => i.to_string(),
			Variant::Float(f) => f.to_string(),
			Variant::Str(s) => s.to_string(),
		}
	}

	pub fn to_bool(&self) -> bool {
		match self {
			Variant::Boolean(b) => *b,
			Variant::Integer(i) => *i != 0,
			Variant::Float(f) => *f != 0.0,
			Variant::Str(s) => !s.is_empty(),
		}
	}

	pub fn to_int(&self) -> Option<i64> {
		match self {
			Variant::Boolean(b) => Some(*b as i64),
			Variant::Integer(i) => Some(*i),
			Variant::Float(f) => Some(*f as i64),
			Variant::Str(s) => s.parse::<i64>().ok(),
		}
	}

	pub fn to_float(&self) -> Option<f64> {
		match self {
			Variant::Boolean(b) => Some(*b as i64 as f64),
			Variant::Integer(i) => Some(*i as f64),
			Variant::Float(f) => Some(*f),
			Variant::Str(s) => s.parse::<f64>().ok(),
		}
	}

	pub fn to_string(&self) -> String {
		match self {
			Variant::Boolean(b) => b.to_string(),
			Variant::Integer(i) => i.to_string(),
			Variant::Float(f) => f.to_string(),
			Variant::Str(s) => s.to_string(),
		}
	}

	pub fn add(&self, rhs: Variant) -> Option<Variant> {
		if let (Some(left), Some(right)) = (self.try_unwrap_bool(), rhs.try_unwrap_bool()) {
			Some(Variant::Boolean(left || right))
		} else if let (Some(left), Some(right)) = (self.try_unwrap_int(), rhs.try_unwrap_int()) {
			Some(Variant::Integer(left + right))
		} else if let (Some(left), Some(right)) = (self.try_unwrap_float(), rhs.try_unwrap_float()) {
			Some(Variant::Float(left + right))
		} else if let (Some(left), Some(right)) = (self.try_unwrap_string(), rhs.try_unwrap_string()) {
			Some(Variant::Str(left + &right))
		} else {
			None
		}
	}

	pub fn sub(&self, rhs: Variant) -> Option<Variant> {
		if let (Some(left), Some(right)) = (self.try_unwrap_int(), rhs.try_unwrap_int()) {
			Some(Variant::Integer(left - right))
		} else if let (Some(left), Some(right)) = (self.try_unwrap_float(), rhs.try_unwrap_float()) {
			Some(Variant::Float(left - right))
		} else if let (Some(left), Some(right)) = (self.try_unwrap_string(), rhs.try_unwrap_int()) {
			if right >= 0 {
				Some(Variant::Str(left[..left.len() - right as usize].to_string()))
			} else {
				None
			}
		} else if let (Some(mut left), Some(right)) = (self.try_unwrap_string(), rhs.try_unwrap_string()) {
			if left.contains(&right) {
				left.remove_matches(&right);
				Some(Variant::Str(left))
			} else {
				None
			}
		} else {
			None
		}
	}

	pub fn mul(&self, rhs: Variant) -> Option<Variant> {
		if let (Some(left), Some(right)) = (self.try_unwrap_int(), rhs.try_unwrap_int()) {
			Some(Variant::Integer(left * right))
		} else if let (Some(left), Some(right)) = (self.try_unwrap_float(), rhs.try_unwrap_float()) {
			Some(Variant::Float(left * right))
		} else if let (Some(left), Some(right)) = (self.try_unwrap_string(), rhs.try_unwrap_int()) {
			if right >= 0 {
				Some(Variant::Str(left.repeat(right as usize)))
			} else {
				None
			}
		} else {
			None
		}
	}

	pub fn div(&self, rhs: Variant) -> Option<Variant> {
		if let (Some(left), Some(right)) = (self.try_unwrap_int(), rhs.try_unwrap_int()) {
			Some(Variant::Integer(left / right))
		} else if let (Some(left), Some(right)) = (self.try_unwrap_float(), rhs.try_unwrap_float()) {
			Some(Variant::Float(left / right))
		} else {
			None
		}
	}

	pub fn inverted(&self) -> Option<Variant> {
		match self {
			Variant::Boolean(b) => Some(Variant::Boolean(!b)),
			Variant::Integer(i) => Some(Variant::Integer(-i)),
			Variant::Float(f) => Some(Variant::Float(-f)),
			Variant::Str(s) => Some(Variant::Str(s.chars().rev().collect())),
		}
	}
}
