// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

pub struct SpellbookError {
	pub message: String,
	pub line: usize,
}

#[macro_export]
macro_rules! sb_panic {
	($message:literal, $line:expr) => {
		//std::panic::panic_any(crate::errors::SpellbookError{message: $message.to_string(), line: $line});
		//panic!();
		std::panic::panic_any($line);
	};
}
