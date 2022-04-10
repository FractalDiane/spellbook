// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

use peekmore::{PeekMore, PeekMoreIterator};

use crate::variant::Variant;
use crate::program::Program;
use crate::sb_panic;
use crate::constants::*;

use std::slice::Iter;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyphrase {
	TurnToChapter,
	TearOutChapter,
	AndPutItInTheDrawer,
	AndThrowItInTheTrash,
	AndTossItInTheCauldron,
	TakeOutAChapterFromTheDrawerAndPutItBack,
	TakeOutChapterFromTheCauldronAndPutItBack,
	//PickUpChapterOffTheFloorAndPutItBack,

	Write,
	Under,
	Copy,
	Entry,

	Memorize,
	FromMemory,

	Cast,
	OnTheCauldron,
	KnockOverCauldron,

	PublishSpellbook,
	PublishSpellbookTo,
	SignChapterWith,
	SignAcknowledgementsPageWith,

	SlamSpellbookShut,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
	Keyphrase(Keyphrase),
	Literal(Variant),
	Identifier(String),
	Builtin(String),

	Conditional,
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
		if let Some(item) = iter.peek_nth(index) {
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

	let words = line.split_whitespace().peekable();
	let nocomments = words.filter(|w| !w.chars().all(|c| c.is_uppercase() || (c.is_ascii_punctuation() && c != '"')));
	let split = split_line_with_quotes(nocomments.collect::<Vec<&str>>().join(" "));
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
			"take" => {
				if expect_subtokens(&mut subtokens, &["out", "a", "chapter", "from", "the", "drawer", "and", "put", "it", "back"]) {
					let token = Token::Keyphrase(Keyphrase::TakeOutAChapterFromTheDrawerAndPutItBack);
					tokens.push(token);
				} else if expect_subtokens(&mut subtokens, &["out", "chapter", "from", "the", "cauldron", "and", "put", "it", "back"]) {
					let token = Token::Keyphrase(Keyphrase::TakeOutChapterFromTheCauldronAndPutItBack);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"write" => {
				let token = Token::Keyphrase(Keyphrase::Write);
				tokens.push(token);
			},
			"copy" => {
				let token = Token::Keyphrase(Keyphrase::Copy);
				tokens.push(token);
			},
			"entry" => {
				let token = Token::Keyphrase(Keyphrase::Entry);
				tokens.push(token);
			},
			"under" => {
				let token = Token::Keyphrase(Keyphrase::Under);
				tokens.push(token);
			},
			"cast" => {
				let token = Token::Keyphrase(Keyphrase::Cast);
				tokens.push(token);
			},
			"on" => {
				if expect_subtokens(&mut subtokens, &["the", "cauldron"]) {
					let token = Token::Keyphrase(Keyphrase::OnTheCauldron);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"knock" => {
				if expect_subtokens(&mut subtokens, &["over", "cauldron"]) {
					let token = Token::Keyphrase(Keyphrase::KnockOverCauldron);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"from" => {
				if expect_subtokens(&mut subtokens, &["memory"]) {
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
				} else if expect_subtokens(&mut subtokens, &["spellbook"]) {
					let token = Token::Keyphrase(Keyphrase::PublishSpellbook);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"and" => {
				match subtokens.peek() {
					Some(&&"put") => {
						if expect_subtokens(&mut subtokens, &["put", "it", "in", "the", "drawer"]) {
							let token = Token::Keyphrase(Keyphrase::AndPutItInTheDrawer);
							tokens.push(token);
						} else {
							return None;
						}
					},
					Some(&&"throw") => {
						if expect_subtokens(&mut subtokens, &["throw", "it", "in", "the", "trash"]) {
							let token = Token::Keyphrase(Keyphrase::AndThrowItInTheTrash);
							tokens.push(token);
						} else {
							return None;
						}
					},
					Some(&&"toss") => {
						if expect_subtokens(&mut subtokens, &["toss", "it", "in", "the", "cauldron"]) {
							let token = Token::Keyphrase(Keyphrase::AndTossItInTheCauldron);
							tokens.push(token);
						} else {
							return None;
						}
					},
					_ => {
						return None;
					},
				}
			},
			"sign" => {
				if expect_subtokens(&mut subtokens, &["chapter", "with"]) {
					let token = Token::Keyphrase(Keyphrase::SignChapterWith);
					tokens.push(token);
				} else if expect_subtokens(&mut subtokens, &["acknowledgements", "page", "with"]) {
					let token = Token::Keyphrase(Keyphrase::SignAcknowledgementsPageWith);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"memorize" => {
				let token = Token::Keyphrase(Keyphrase::Memorize);
				tokens.push(token);
			},
			"slam" => {
				if expect_subtokens(&mut subtokens, &["spellbook", "shut"]) {
					let token = Token::Keyphrase(Keyphrase::SlamSpellbookShut);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"if" => {
				let token = Token::Conditional;
				tokens.push(token);
			},

			_ => {
				if BUILTINS_ORDINALS.contains(st)
				|| BUILTINS_CHAPTERS.contains(st)
				|| BUILTINS_MISC.contains(st)
				|| BUILTINS_SPELLS.contains_key(st) {
					tokens.push(Token::Builtin(st.to_string()));
				} else if let Ok(int) = st.parse::<i64>() {
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
}

struct ParserState {
	status: ParseStateStatus,
	cached_keyphrase: Option<Keyphrase>,
	cached_identifier: String,
	cached_builtin: String,
	cached_literal: Option<Variant>,
	cached_operand_list: Vec<Variant>,
}

impl ParserState {
	pub fn new() -> Self {
		Self{
			status: ParseStateStatus::Top,
			cached_keyphrase: None,
			cached_identifier: String::with_capacity(20),
			cached_builtin: String::with_capacity(20),
			cached_literal: None,
			cached_operand_list: Vec::<Variant>::with_capacity(5),
		}
	}

	pub fn clear_cache(&mut self) {
		self.cached_keyphrase = None;
		self.cached_identifier.clear();
		self.cached_builtin.clear();
		self.cached_literal = None;
		self.cached_operand_list.clear();
		self.status = ParseStateStatus::Top;
	}

	pub fn is_cache_clear(&self) -> bool {
		self.cached_keyphrase.is_none()
		&& self.cached_identifier.is_empty()
		&& self.cached_builtin.is_empty()
		&& self.cached_literal.is_none()
		&& self.cached_operand_list.is_empty()
		&& self.status == ParseStateStatus::Top
	}
}

pub fn execute_token_vector(program: &mut Program, tokens: Vec<Token>) {
	if program.debug_mode {
		eprintln!("{:<4}  {:?}", program.line_number, tokens);
	}
	
	let mut state = ParserState::new();

	for token in &tokens {
		execute_token(&token, &mut state, program);
	}

	while !state.is_cache_clear() {
		execute_token(&tokens.last().unwrap(), &mut state, program);
	}
}

fn execute_token(current: &Token, state: &mut ParserState, program: &mut Program) {
	match &state.status {
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
							match BUILTINS_CHAPTERS.iter().position(|&s| s == chapter) {
								Some(index) => {
									program.turn_to_page(index);
								},
								None => {
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
				Keyphrase::Write => {
					match current {
						Token::Literal(lit) => {
							state.cached_literal = Some(lit.clone());
							state.cached_keyphrase = Some(Keyphrase::Write);
							state.status = ParseStateStatus::Top;
						},
						Token::Keyphrase(kp) => {
							if *kp == Keyphrase::FromMemory {
								state.cached_keyphrase = Some(Keyphrase::FromMemory);
								state.status = ParseStateStatus::Top;
							} else {
								sb_panic!(program.line_number);
							}
						},
						_ => {
							sb_panic!(program.line_number);
						},
					}
				},
				Keyphrase::Copy => {
					match current {
						Token::Identifier(ident) => {
							state.cached_identifier = ident.clone();
							state.cached_keyphrase = Some(Keyphrase::Copy);
							state.status = ParseStateStatus::Top;
						},
						_ => {
							sb_panic!(program.line_number);
						}
					}
				},
				Keyphrase::Under => {
					match current {
						Token::Identifier(ident) => {
							match state.cached_keyphrase {
								Some(Keyphrase::Write) => {
									program.write_literal_value(ident.clone(), state.cached_literal.clone());
									state.clear_cache();
								},
								Some(Keyphrase::Copy) => {
									match program.try_get_value(&state.cached_identifier) {
										Some(val) => {
											program.write_literal_value(ident.clone(), Some(val));
										},
										None => {
											sb_panic!(program.line_number);
										},
									}
								},
								Some(Keyphrase::FromMemory) => {
									program.write_memory_value(ident.clone());
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
				Keyphrase::SignChapterWith => {
					match current {
						Token::Literal(lit) => {
							match lit {
								Variant::Str(string) => {
									program.sign_page(string.clone());
									state.clear_cache();
								},
								_ => {
									sb_panic!(program.line_number);
								}
							}
						},
						_ => {
							sb_panic!(program.line_number);
						},
					}
				},
				Keyphrase::TearOutChapter => {
					state.cached_keyphrase = Some(Keyphrase::TearOutChapter);
					state.status = ParseStateStatus::Top;
				},
				Keyphrase::AndThrowItInTheTrash => {
					if state.cached_keyphrase == Some(Keyphrase::TearOutChapter) {
						program.tear_out_page(false, false);
						state.clear_cache();
					} else {
						sb_panic!(program.line_number);
					}
				},
				Keyphrase::AndPutItInTheDrawer => {
					if state.cached_keyphrase == Some(Keyphrase::TearOutChapter) {
						program.tear_out_page(true, false);
						state.clear_cache();
					} else {
						sb_panic!(program.line_number);
					}
				},
				Keyphrase::AndTossItInTheCauldron => {
					if state.cached_keyphrase == Some(Keyphrase::TearOutChapter) {
						program.tear_out_page(false, true);
						state.clear_cache();
					} else {
						sb_panic!(program.line_number);
					}
				},
				Keyphrase::TakeOutAChapterFromTheDrawerAndPutItBack => {
					program.put_back_page(true);
					state.clear_cache();
				},
				Keyphrase::TakeOutChapterFromTheCauldronAndPutItBack => {
					program.put_back_page(false);
					state.clear_cache();
				},
				Keyphrase::Cast => {
					match current {
						Token::Builtin(bt) => {
							state.cached_builtin = bt.clone();
							state.status = ParseStateStatus::Top;
						},
						_ => {
							sb_panic!(program.line_number);
						},
					}
				},
				Keyphrase::OnTheCauldron => {
					program.cast_cauldron_spell(&BUILTINS_SPELLS[state.cached_builtin.as_str()]);
					state.clear_cache();
				},
				Keyphrase::KnockOverCauldron => {
					program.knock_over_cauldron();
					state.clear_cache();
				},
				Keyphrase::PublishSpellbook => {
					program.publish(false, String::new());
					state.clear_cache();
				},
				Keyphrase::PublishSpellbookTo => {
					match current {
						Token::Literal(lit) => {
							match lit {
								Variant::Str(string) => {
									program.publish(true, string.clone());
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
				Keyphrase::SignAcknowledgementsPageWith => {
					match current {
						Token::Literal(lit) => {
							match lit {
								Variant::Str(string) => {
									program.set_signature(string.clone());
									state.clear_cache();
								},
								_ => {
									sb_panic!(program.line_number);
								}
							}
						},
						_ => {
							sb_panic!(program.line_number);
						}
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
						Token::Builtin(bt) => {
							state.cached_keyphrase = Some(Keyphrase::Memorize);
							state.cached_builtin = bt.clone();	
							state.status = ParseStateStatus::Top;
						},
						_ => {
							sb_panic!(program.line_number);
						},
					}
				},
				Keyphrase::Entry => {
					match state.cached_keyphrase {
						Some(Keyphrase::Memorize) => {
							let index = match state.cached_builtin.as_str() {
								"first" => 0,
								"second" => 1,
								"third" => 2,
								_ => {
									sb_panic!(program.line_number);
								},
							};

							program.memorize_value(program.get_value_by_index(index));
							state.clear_cache();
						},
						_ => {
							sb_panic!(program.line_number);
						},
					}
				},
				Keyphrase::SlamSpellbookShut => {
					program.exit = true;
					state.clear_cache();
				},

				Keyphrase::FromMemory => {},
			}
		},
	}
}
