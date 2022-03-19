// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

use std::str::SplitWhitespace;
use std::thread::current;

use crate::variant::Variant;
use crate::Program;
use crate::page::PageType;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyphrase {
	TurnToChapter,
	TearOutChapter,
	AndPutItInTheDrawer,
	AndThrowItInTheTrash,

	WriteEntry,
	WithValue,
	WithThe,
	WithTheValueOf,
	FromDivineIntervention,
	

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

const fn parse_escape_character(chr: char) -> Option<char> {
	match chr {
		'\\' => Some('\\'),
		'"' => Some('"'),
		'n' => Some('\n'),
		'r' => Some('\r'),
		't' => Some('\t'),
		'0' => Some('\0'),
		_ => None,
	}
}

fn split_line_with_quotes(line: String) -> Vec<String> {
	let mut vec = vec![];
	let mut current_word = String::with_capacity(50);
	let mut in_quotes = false;

	let mut chars = line.chars();
	while let Some(chr) = chars.next()  {
		if chr.is_whitespace() {
			if !in_quotes && !current_word.is_empty() {
				vec.push(current_word.clone());
				current_word.clear();
			} else if in_quotes {
				current_word.push(chr);
			}
		} else if chr == '"' {
			current_word.push(chr);
			in_quotes ^= true;
			if !in_quotes {
				vec.push(current_word.clone());
				current_word.clear();
			}
		} else if chr == '\\' {
			if let Some(esc) = parse_escape_character(chars.next().unwrap()) {
				current_word.push(esc);
			}
		} else {
			current_word.push(chr);
		}
	}

	if !current_word.is_empty() {
		vec.push(current_word);
	}

	vec
}

pub fn tokenize_line(line: String) -> Option<Vec<Token>> {
	let mut tokens = vec![];
	let split = split_line_with_quotes(line);
	let mut subtokens = split.iter().map(|s| &**s);
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
			"write" => {
				if subtokens.next() == Some("entry") {
					let token = Token::Keyphrase(Keyphrase::WriteEntry);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"with" => {
				match subtokens.next() {
					Some("value") => {
						let token = Token::Keyphrase(Keyphrase::WithValue);
						tokens.push(token);
					},
					Some("the") => {
						if subtokens.next() == Some("value") && subtokens.next() == Some("of") {
							let token = Token::Keyphrase(Keyphrase::WithTheValueOf);
							tokens.push(token);
						} else {
							return None;
						}
					},
					_ => {
						return None;
					}
				}
			},
			"from" => {
				if subtokens.next() == Some("divine") && subtokens.next() == Some("intervention") {
					let token = Token::Keyphrase(Keyphrase::FromDivineIntervention);
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
			"and" => {
				match subtokens.next() {
					Some("put") => {
						if subtokens.next() == Some("it") && subtokens.next() == Some("in") && subtokens.next() == Some("the") && subtokens.next() == Some("drawer") {
							let token = Token::Keyphrase(Keyphrase::AndPutItInTheDrawer);
							tokens.push(token);
						} else {
							return None;
						}
					},
					Some("throw") => {
						if subtokens.next() == Some("it") && subtokens.next() == Some("in") && subtokens.next() == Some("the") && subtokens.next() == Some("trash") {
							let token = Token::Keyphrase(Keyphrase::AndThrowItInTheTrash);
							tokens.push(token);
						} else {
							return None;
						}
					},
					_ => {
						return None;
					},
				}
			}
			"wrapped" => {
				if subtokens.next() == Some("up") && subtokens.next() == Some("with") {
					let token = Token::Keyphrase(Keyphrase::WrappedUpWith);
					tokens.push(token);
				} else {
					return None;
				}
			},

			"Presages" | "Hexes" | "Illusions" | "Incantations" | "Recipes" |
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
					let token = Token::Literal(Variant::Str(st[1..st.len() - 1].into()));
					tokens.push(token);
				} else {
					let token = Token::Identifier(st.to_string());
					tokens.push(token);
				}
			},
		}
	}

	Some(tokens)
}

struct ParserState {
	status: ParseStateStatus,
	previous_status: ParseStateStatus,

	cached_keyphrase: Option<Keyphrase>,
	cached_identifier: String,
	cached_builtin: String,
}

impl ParserState {
	pub fn new() -> Self {
		Self{
			status: ParseStateStatus::Top,
			previous_status: ParseStateStatus::Null,
			cached_keyphrase: None,
			cached_identifier: String::new(),
			cached_builtin: String::new(),
		}
	}

	pub fn clear_cache(&mut self) {
		self.cached_keyphrase = None;
		self.cached_identifier.clear();
		self.cached_builtin.clear();
		self.status = ParseStateStatus::Top;
	}

	pub fn is_cache_clear(&self) -> bool {
		self.cached_keyphrase.is_none()
		&& self.cached_identifier.is_empty()
		&& self.cached_builtin.is_empty()
		&& self.status == ParseStateStatus::Top
	}
}

#[derive(PartialEq)]
enum ParseStateStatus {
	Null,
	Top,
	Keyphrase(Keyphrase),
	Operation,
}

pub fn execute_token_vector(program: &mut Program, tokens: Vec<Token>) -> bool {
	let mut state = ParserState::new();
	let mut prev_iter = tokens.iter();
	prev_iter.next().unwrap();
	for (current, next) in tokens.iter().zip(prev_iter) {
		if !execute_tokens(current, Some(next), &mut state, program) {
			return false;
		}
	}

	let final_success = execute_tokens(tokens.as_slice().last().unwrap(), None, &mut state, program);
	final_success && state.is_cache_clear()
}

fn execute_tokens(current: &Token, next: Option<&Token>, state: &mut ParserState, program: &mut Program) -> bool {
	match state.status {
		ParseStateStatus::Top => {
			match current {
				Token::Keyphrase(kp) => {
					state.status = ParseStateStatus::Keyphrase(kp.clone());
				},
				_ => {
					return false;
				},
			}
		},

		ParseStateStatus::Keyphrase(ref kp) => {
			match kp {
				Keyphrase::TurnToChapter => {
					match current {
						Token::Builtin(chapter) => {
							match chapter.as_str() {
								"Presages" => {
									program.turn_to_page(PageType::Boolean as usize);
								},
								"Hexes" => {
									program.turn_to_page(PageType::Integer as usize);
								},
								"Illusions" => {
									program.turn_to_page(PageType::Float as usize);
								},
								"Incantations" => {
									program.turn_to_page(PageType::Str as usize);
								},
								"Recipes" => {
									program.turn_to_page(PageType::Routine as usize);
								},
								_ => {
									return false;
								},
							}

							state.clear_cache();
						},
						_ => {
							return false;
						},
					}
				},
				Keyphrase::WriteEntry => {
					match current {
						Token::Identifier(ident) => {
							state.cached_identifier = ident.to_string();
							state.cached_keyphrase = Some(Keyphrase::WriteEntry);
							state.status = ParseStateStatus::Top;
						},
						_ => {
							return false;
						},
					}
				},
				Keyphrase::WithValue => {
					match current {
						Token::Literal(lit) => {
							if state.cached_keyphrase == Some(Keyphrase::WriteEntry) {
								program.write_literal_value(Some(lit.clone()));
								state.clear_cache();
							} else {
								return false;
							}
						},
						_ => {
							return false;
						},
					}
				},
				Keyphrase::PublishSpellbookTo => {
					match current {
						Token::Builtin(bt) => {
							if bt == "console" {
								state.cached_builtin = bt.to_string();
							} else {
								return false;
							}
						},
						Token::Identifier(ident) => {
							state.cached_identifier = ident.to_string();
						},
						_ => {
							return false;
						},
					}

					if next.is_none() {
						program.publish(
							if state.cached_identifier.is_empty() {
								state.cached_builtin.clone()
							} else { 
								state.cached_identifier.clone()
							},
							false,
							String::new(),
						);

						state.clear_cache();
					} else {
						state.cached_keyphrase = Some(Keyphrase::PublishSpellbookTo);
						state.status = ParseStateStatus::Top;
					}
				},
				Keyphrase::WrappedUpWith => {
					match current {
						Token::Literal(lit) => {
							match lit {
								Variant::Str(st) => {
									program.publish(
										if state.cached_identifier.is_empty() {
											state.cached_builtin.clone()
										} else { 
											state.cached_identifier.clone()
										},
										true,
										st.to_string(),
									);

									state.clear_cache();
								},
								_ => {
									return false;
								},
							}
						},
						_ => {
							return false;
						},
					}
				},
				_ => {},
			}
		},

		_ => {},
	}

	true
}

#[test]
fn test_split_line_with_quotes() {
	assert_eq!(
		split_line_with_quotes(r#"write entry hello with value "Hello, world!" and other stuff"#.into()),
		vec!["write".to_string(), "entry".into(), "hello".into(), "with".into(), "value".into(), 
				r#""Hello, world!""#.into(), "and".into(), "other".into(), "stuff".into(),
		],
	);
}

#[test]
fn test_tokenize() {
	type T = Token;
	type K = Keyphrase;

	assert_eq!(split_line_with_quotes(r#"Hello!\n"#.into()), vec!["Hello!\n"]);
	assert_eq!(split_line_with_quotes(r#"These \xare \finvalid \yescapes"#.into()), vec!["These", "are", "invalid", "escapes"]);
	
	assert_eq!(tokenize_line("turn to chapter".into()), Some(vec![T::Keyphrase(K::TurnToChapter)]));
	assert_eq!(tokenize_line("turn to page".into()), None);

	assert_eq!(tokenize_line("turn to chapter Incantations".into()), Some(vec![
		T::Keyphrase(K::TurnToChapter),
		T::Builtin("Incantations".into()),
	]));

	assert_eq!(
		tokenize_line(r#""Hello, world!""#.into()),
		Some(vec![T::Literal(Variant::Str(r#"Hello, world!"#.into()))]),
	);

	assert_eq!(tokenize_line("write entry hello with value 5".into()), Some(vec![
		T::Keyphrase(K::WriteEntry),
		T::Identifier("hello".into()),
		T::Keyphrase(K::WithValue),
		T::Literal(Variant::Integer(5)),
	]));
}
