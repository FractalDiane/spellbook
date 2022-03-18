// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

use crate::variant::Variant;
use crate::Program;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyphrase {
	TurnToChapter,
	TearOutChapter,
	PutItInTheDrawer,
	ThrowItInTheTrash,
	

	PublishSpellbookTo,
	WrappedUpWith,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
	Keyphrase(Keyphrase),
	Literal(Variant),
	Identifier(String),
	Builtin(String),
	Operator(String),
}

pub fn tokenize_line(line: String) -> Option<Vec<Token>> {
	let mut tokens = vec![];
	let mut subtokens = line.split_whitespace().peekable();
	while let Some(st) = subtokens.next() {
		match st {
			"turn" => {
				if subtokens.next() == Some("to") && subtokens.next() == Some("chapter") {
					let token = Token::Keyphrase(Keyphrase::TurnToChapter);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"tear" => {
				if subtokens.next() == Some("out") && subtokens.next() == Some("chapter") {
					let token = Token::Keyphrase(Keyphrase::TearOutChapter);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"publish" => {
				if subtokens.next() == Some("spellbook") && subtokens.next() == Some("to") {
					let token = Token::Keyphrase(Keyphrase::PublishSpellbookTo);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"wrapped" => {
				if subtokens.next() == Some("up") && subtokens.next() == Some("with") {
					let token = Token::Keyphrase(Keyphrase::WrappedUpWith);
					tokens.push(token);
				} else {
					return None;
				}
			},

			"Incantations" | "Recipes" |
			"console" => {
				tokens.push(Token::Builtin(st.into()));
			},

			_ => {
				if let Ok(int) = st.parse::<i64>() {
					let token = Token::Literal(Variant::Integer(int));
					tokens.push(token);
				} else if let Ok(float) = st.parse::<f64>() {
					let token = Token::Literal(Variant::Float(float));
					tokens.push(token);
				} else if let Ok(boolean) = st.parse::<bool>() {
					let token = Token::Literal(Variant::Boolean(boolean));
					tokens.push(token);
				} else if st.starts_with('"') {
					if let Some(end) = st[1..].find('"') {
						let token = Token::Literal(Variant::Str(st[1..end].into()));
						tokens.push(token);
					} else {
						return None;
					}
				} else {
					return None;
				}
			},
		}
	}

	Some(tokens)
}

enum ParseState {
	Top,
	Keyphrase(Keyphrase),
	Operation,
}

pub fn execute_token_vector(program: &mut Program, tokens: Vec<Token>) -> bool {
	let mut state = ParseState::Top;
	let mut prev_iter = tokens.iter();
	prev_iter.next().unwrap();
	for (current, next) in tokens.iter().zip(prev_iter) {
		match state {
			ParseState::Top => {
				match current {
					Token::Keyphrase(kp) => {
						state = ParseState::Keyphrase(kp.clone());
					},
					_ => {
						panic!();
					},
				}
			},

			ParseState::Keyphrase(ref kp) => {
				match kp {
					Keyphrase::TurnToChapter => {
						match current {
							Token::Builtin(chapter) => {
								match chapter.as_str() {
									"Presages" => {

									},
									"Hexes" => {

									},
									"Illusions" => {

									},
									"Incantations" => {

									},
									"Recipes" => {

									},
									_ => {
										panic!();
									},
								}
							},
							_ => {
								panic!();
							},
						}
					},
					_ => {},
				}
			},

			_ => {},
		}
	}

	true
}

#[test]
fn test_tokenize() {
	type T = Token;
	type K = Keyphrase;
	assert_eq!(tokenize_line("turn to chapter".into()), Some(vec![T::Keyphrase(K::TurnToChapter)]));
	assert_eq!(tokenize_line("turn to page".into()), None);

	assert_eq!(tokenize_line("turn to chapter Incantations".into()), Some(vec![
		T::Keyphrase(K::TurnToChapter),
		T::Builtin("Incantations".into()),
	]));
}
