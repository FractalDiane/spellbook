// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

#[allow(unused)]
pub struct SpellbookError {
	pub message: String,
	pub line: usize,
}

#[macro_export]
macro_rules! sb_panic {
	($line:expr) => {
		std::panic::panic_any($line);
	};
}
