// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

pub const ERROR_MESSAGES: [&'static str; 12] = [
	"You accidentally transformed yourself into a cute, fluffy demon.",
	"The spell ricocheted off the wall and knocked everything off your desk.",
	"You appear to have set yourself on fire.",
	"KABOOM! Something exploded that definitely wasn't supposed to explode.",
	"Your room is suddenly filled with demons you summoned by accident. They look unamused.",
	"The spell fizzled and did nothing.",
	"You got tongue-tied while you were saying the spell.",
	"Whoops! You dropped the spellbook and it blasted a hole in the floor.",
	"Something went wrong, and now it's raining inside your house.",
	"The spell did something, and you aren't entirely sure what, but it's absolutely not correct.",
	"The spellbook burst into flames and burned to ashes.",
	"You summoned some fireworks that blew up a bunch of stuff. But they sure looked cool!",
];

pub const DEFAULT_WRAPUP: &'static str = "\n*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
               THE END  
*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*\n";

pub const DEFAULT_WRAPUP_QED: &'static str = "\n*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
                Q.E.D.
*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*\n";
