// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

#[macro_export]
macro_rules! sb_panic {
	($line:expr) => {
		std::panic::panic_any($line);
		//panic!();
	};
}
