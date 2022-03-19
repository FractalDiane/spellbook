// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

use peekmore::{PeekMore, PeekMoreIterator};

use crate::variant::Variant;
use crate::Program;
use crate::page::PageType;
use crate::sb_panic;

use std::slice::Iter;

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

	Memorize,
	FromMemory,
	

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

fn expect_subtokens(iter: &mut PeekMoreIterator<Iter<&str>>, subtokens: &[&str]) -> bool {
	let mut index = 0;
	for st in subtokens {
		if let Some(item) = iter.peek_forward(index) {
			if st != *item {
				return false;
			}
		} else {
			return false;
		}

		index += 1;
	}

	iter.advance_by(subtokens.len()).unwrap();
	true
}

pub fn tokenize_line(line: String) -> Option<Vec<Token>> {
	let mut tokens = vec![];
	let split = split_line_with_quotes(line);
	let vec = split.iter().map(|s| &**s).collect::<Vec<&str>>();
	let mut subtokens = vec.iter().peekmore();
	while let Some(st) = subtokens.next() {
		match *st {
			"turn" => {
				if expect_subtokens(&mut subtokens, &["to", "chapter"]) {
					let token = Token::Keyphrase(Keyphrase::TurnToChapter);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"tear" => {
				if expect_subtokens(&mut subtokens, &["out", "chapter"]) {
					let token = Token::Keyphrase(Keyphrase::TearOutChapter);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"write" => {
				if expect_subtokens(&mut subtokens, &["entry"]) {
					let token = Token::Keyphrase(Keyphrase::WriteEntry);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"with" => {
				match subtokens.next() {
					Some(&"value") => {
						let token = Token::Keyphrase(Keyphrase::WithValue);
						tokens.push(token);
					},
					Some(&"the") => {
						if expect_subtokens(&mut subtokens, &["value", "of"]) {
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
				if expect_subtokens(&mut subtokens, &["divine", "intervention"]) {
					let token = Token::Keyphrase(Keyphrase::FromDivineIntervention);
					tokens.push(token);
				} else if expect_subtokens(&mut subtokens, &["memory"]) {
					let token = Token::Keyphrase(Keyphrase::FromMemory);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"publish" => {
				if expect_subtokens(&mut subtokens, &["spellbook", "to"]) {
					let token = Token::Keyphrase(Keyphrase::PublishSpellbookTo);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"and" => {
				match subtokens.next() {
					Some(&"put") => {
						if expect_subtokens(&mut subtokens, &["it", "in", "the", "drawer"]) {
							let token = Token::Keyphrase(Keyphrase::AndPutItInTheDrawer);
							tokens.push(token);
						} else {
							return None;
						}
					},
					Some(&"throw") => {
						if expect_subtokens(&mut subtokens, &["it", "in", "the", "trash"]) {
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
				if expect_subtokens(&mut subtokens, &["up", "with"]) {
					let token = Token::Keyphrase(Keyphrase::WrappedUpWith);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"memorize" => {
				let token = Token::Keyphrase(Keyphrase::Memorize);
				tokens.push(token);
			},

			"Presages" | "Hexes" | "Illusions" | "Incantations" | "Recipes" |
			"console" => {
				tokens.push(Token::Builtin(st.to_string()));
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

#[derive(PartialEq)]
enum ParseStateStatus {
	Top,
	Keyphrase(Keyphrase),
	Operation,
}

struct ParserState {
	status: ParseStateStatus,
	cached_keyphrase: Option<Keyphrase>,
	cached_identifier: String,
	cached_builtin: String,
	cached_literal: String,
}

impl ParserState {
	pub fn new() -> Self {
		Self{
			status: ParseStateStatus::Top,
			cached_keyphrase: None,
			cached_identifier: String::with_capacity(20),
			cached_builtin: String::with_capacity(20),
			cached_literal: String::with_capacity(20),
		}
	}

	pub fn clear_cache(&mut self) {
		self.cached_keyphrase = None;
		self.cached_identifier.clear();
		self.cached_builtin.clear();
		self.cached_literal.clear();
		self.status = ParseStateStatus::Top;
	}

	pub fn is_cache_clear(&self) -> bool {
		self.cached_keyphrase.is_none()
		&& self.cached_identifier.is_empty()
		&& self.cached_builtin.is_empty()
		&& self.cached_literal.is_empty()
		&& self.status == ParseStateStatus::Top
	}
}

pub fn execute_token_vector(program: &mut Program, tokens: Vec<Token>) {
	println!("{:?}", tokens);
	let mut state = ParserState::new();
	let mut prev_iter = tokens.iter();
	prev_iter.next().unwrap();
	for (current, next) in tokens.iter().zip(prev_iter) {
		execute_tokens(current, Some(next), &mut state, program);
	}

	while !state.is_cache_clear() {
		execute_tokens(tokens.as_slice().last().unwrap(), None, &mut state, program);
	}
}

fn execute_tokens(current: &Token, next: Option<&Token>, state: &mut ParserState, program: &mut Program) {
	match state.status {
		ParseStateStatus::Top => {
			match current {
				Token::Keyphrase(kp) => {
					state.status = ParseStateStatus::Keyphrase(kp.clone());
				},
				_ => {
					sb_panic!(program.line_number);
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
									sb_panic!(program.line_number);
								},
							}

							state.clear_cache();
						},
						_ => {
							sb_panic!(program.line_number);
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
							sb_panic!(program.line_number);
						},
					}
				},
				Keyphrase::WithValue => {
					match current {
						Token::Literal(lit) => {
							if state.cached_keyphrase == Some(Keyphrase::WriteEntry) {
								program.write_literal_value(state.cached_identifier.clone(), Some(lit.clone()));
								state.clear_cache();
							} else {
								sb_panic!(program.line_number);
							}
						},
						_ => {
							sb_panic!(program.line_number);
						},
					}
				},
				Keyphrase::FromMemory => {
					if state.cached_keyphrase == Some(Keyphrase::WriteEntry) {
						program.write_memory_value(state.cached_identifier.clone());
						state.clear_cache();
					} else {
						sb_panic!(program.line_number);
					}
				},
				Keyphrase::PublishSpellbookTo => {
					match current {
						Token::Builtin(bt) => {
							if bt == "console" {
								state.cached_builtin = bt.to_string();
							} else {
								sb_panic!(program.line_number);
							}
						},
						Token::Literal(lit) => {
							match lit {
								Variant::Str(string) => {
									state.cached_literal = string.clone();
								},
								_ => {
									sb_panic!(program.line_number);
								},
							}
						},
						Token::Identifier(ident) => {
							state.cached_identifier = ident.to_string();
						},
						_ => {
							sb_panic!(program.line_number);
						},
					}

					if next.is_none() {
						let not_console = &state.cached_builtin != "console";
						program.publish(
							not_console,
							if !not_console {
								String::new()
							} else if !state.cached_literal.is_empty() {
								state.cached_literal.clone()
							} else {
								match program.try_get_value(&state.cached_identifier) {
									Some(val) => {
										match val {
											Variant::Str(string) => string,
											_ => {
												sb_panic!(program.line_number);
											},
										}
									},
									None => {
										sb_panic!(program.line_number);
									},
								}
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
									let not_console = &state.cached_builtin != "console";
									program.publish(
										not_console,
										if !not_console {
											String::new()
										} else if !state.cached_literal.is_empty() {
											state.cached_literal.clone()
										} else {
											match program.try_get_value(&state.cached_identifier) {
												Some(val) => {
													match val {
														Variant::Str(string) => string,
														_ => {
															sb_panic!(program.line_number);
														},
													}
												},
												None => {
													println!("{}", state.cached_identifier);
													sb_panic!(program.line_number);
												},
											}
										},
										true,
										st.to_string(),
									);

									state.clear_cache();
								},
								_ => {
									sb_panic!(program.line_number);
								},
							}
						},
						_ => {
							sb_panic!(program.line_number);
						},
					}
				},
				Keyphrase::Memorize => {
					match current {
						Token::Identifier(ident) => {
							match program.try_get_value(&ident) {
								Some(val) => {
									program.memorize_value(Some(val));
								},
								None => {
									sb_panic!(program.line_number);
								},
							}

							state.clear_cache();
						},
						Token::Literal(lit) => {
							program.memorize_value(Some(lit.clone()));
							state.clear_cache();
						},
						_ => {
							sb_panic!(program.line_number);
						},
					}
				},
				_ => {},
			}
		},

		_ => {},
	}
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
