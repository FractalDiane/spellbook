// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

use peekmore::{PeekMore, PeekMoreIterator};

use crate::variant::Variant;
use crate::Program;
use crate::sb_panic;
use crate::constants::*;
use crate::cauldron::CauldronSpell;

use std::slice::Iter;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyphrase {
	TurnToChapter,
	TearOutChapter,
	AndPutItInTheDrawer,
	AndThrowItInTheTrash,
	AndTossItInTheCauldron,
	TakeOutAChapterFromTheDrawerAndPutItBack,
	PickUpChapterOffTheFloorAndPutItBack,

	//WriteEntry,
	//WithValue,
	Write,
	Under,
	Copy,
	//WithThe,
	//WithTheValueOf,
	FromDivineIntervention,
	Entry,

	Memorize,
	FromMemory,

	Cast,
	OnTheCauldron,
	KnockOverCauldron,

	SkipTheNext,
	RepeatTheLast,
	//Steps,

	//PublishSpellbookTo,
	PublishSpellbook,
	//SignChapterWith,
	SignAcknowledgementsPageWith,
	//WrappedUpWith,

	ThrowSpellbookInTheTrash,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
	Keyphrase(Keyphrase),
	Literal(Variant),
	Identifier(String),
	Builtin(String),

	Conditional,
	Operator(Operator),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
	Sum,
	Difference,
	Product,
	Quotient,
	Remainder,
	Concatenation,
	And,
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
	let nocaps = line.split_whitespace().
		filter(|w| !w.chars().all(|c| c.is_uppercase() || !c.is_alphanumeric()) || w.contains('"'))
		.collect::<Vec<&str>>().join(" ");

	let split = split_line_with_quotes(nocaps);
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
				} else {
					return None;
				}
			},
			"pick" => {
				if expect_subtokens(&mut subtokens, &["up", "chapter", "off", "the", "floor", "and", "put", "it", "back"]) {
					let token = Token::Keyphrase(Keyphrase::PickUpChapterOffTheFloorAndPutItBack);
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
			/*"with" => {
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
							let token = Token::Keyphrase(Keyphrase::WithThe);
							tokens.push(token);
						}
					},
					_ => {
						return None;
					}
				}
			},*/
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
			"skip" => {
				if expect_subtokens(&mut subtokens, &["the", "next"]) {
					let token = Token::Keyphrase(Keyphrase::SkipTheNext);
					tokens.push(token);
				} else {
					return None;
				}
			},
			"repeat" => {
				if expect_subtokens(&mut subtokens, &["the", "last"]) {
					let token = Token::Keyphrase(Keyphrase::RepeatTheLast);
					tokens.push(token);
				}
			},
			"publish" => {
				if expect_subtokens(&mut subtokens, &["spellbook"]) {
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
						let token = Token::Operator(Operator::And);
						tokens.push(token);
					},
				}
			},
			/*"wrapped" => {
				if expect_subtokens(&mut subtokens, &["up", "with"]) {
					let token = Token::Keyphrase(Keyphrase::WrappedUpWith);
					tokens.push(token);
				} else {
					return None;
				}
			},*/
			"sign" => {
				/*if expect_subtokens(&mut subtokens, &["chapter", "with"]) {
					let token = Token::Keyphrase(Keyphrase::SignChapterWith);
					tokens.push(token);
				} else*/ if expect_subtokens(&mut subtokens, &["acknowledgements", "page", "with"]) {
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
			"throw" => {
				if expect_subtokens(&mut subtokens, &["spellbook", "in", "the", "trash"]) {
					let token = Token::Keyphrase(Keyphrase::ThrowSpellbookInTheTrash);
					tokens.push(token);
				}
			},

			"if" => {
				let token = Token::Conditional;
				tokens.push(token);
			},

			"sum" => {
				if expect_subtokens(&mut subtokens, &["of"]) {
					let token = Token::Operator(Operator::Sum);
					tokens.push(token);
				}
			},
			"difference" => {
				if expect_subtokens(&mut subtokens, &["of"]) {
					let token = Token::Operator(Operator::Difference);
					tokens.push(token);
				}
			},
			"product" => {
				if expect_subtokens(&mut subtokens, &["of"]) {
					let token = Token::Operator(Operator::Product);
					tokens.push(token);
				}
			},
			"quotient" => {
				if expect_subtokens(&mut subtokens, &["of"]) {
					let token = Token::Operator(Operator::Quotient);
					tokens.push(token);
				}
			},
			"remainder" => {
				if expect_subtokens(&mut subtokens, &["of"]) {
					let token = Token::Operator(Operator::Remainder);
					tokens.push(token);
				}
			},
			"concatenation" => {
				if expect_subtokens(&mut subtokens, &["of"]) {
					let token = Token::Operator(Operator::Concatenation);
					tokens.push(token);
				}
			},

			_ => {
				if BUILTINS_ORDINALS.contains(st) || BUILTINS_CHAPTERS.contains(st) || BUILTINS_MISC.contains(st) || BUILTINS_SPELLS.contains(st) {
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
	//Operation(Operator),
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

	/* 
	pub fn perform_operation(&self, operation: &Operator) -> Option<Variant> {
		if self.cached_operand_list.len() < 1 {
			return None;
		}

		if let Some(mut result) = match operation {
			Operator::Sum => self.cached_operand_list[0].add(self.cached_operand_list[1].clone()),
			Operator::Difference => self.cached_operand_list[0].sub(self.cached_operand_list[1].clone()),
			Operator::Product => self.cached_operand_list[0].mul(self.cached_operand_list[1].clone()),
			Operator::Quotient => self.cached_operand_list[0].div(self.cached_operand_list[1].clone()),
			Operator::Remainder => self.cached_operand_list[0].rem(self.cached_operand_list[1].clone()),
			Operator::Concatenation => self.cached_operand_list[0].concat(self.cached_operand_list[1].clone()),
			_ => {
				return None;
			}
		} {
			let mut index = 2;
			while index < self.cached_operand_list.len() {
				result = match match operation {
					Operator::Sum => result.add(self.cached_operand_list[index].clone()),
					Operator::Difference => result.sub(self.cached_operand_list[index].clone()),
					Operator::Product => result.mul(self.cached_operand_list[index].clone()),
					Operator::Quotient => result.div(self.cached_operand_list[index].clone()),
					Operator::Remainder => result.rem(self.cached_operand_list[index].clone()),
					Operator::Concatenation => result.concat(self.cached_operand_list[index].clone()),
					_ => {
						return None;
					},
				} {
					Some(r) => r,
					None => {
						return None;
					},
				};

				index += 1;
			}

			Some(result)
		} else {
			None
		}
	}
	*/
}

pub fn execute_token_vector(program: &mut Program, tokens: Vec<Token>) {
	//println!("{:?}", tokens);
	let mut state = ParserState::new();

	if tokens.len() > 1 {
		let mut prev_iter = tokens.iter();
		prev_iter.next().unwrap();
		for (current, next) in tokens.iter().zip(prev_iter) {
			execute_tokens(current, Some(next), &mut state, program, false);
		}

		while !state.is_cache_clear() {
			execute_tokens(tokens.as_slice().last().unwrap(), None, &mut state, program, false);
		}
	} else {
		execute_tokens(&tokens[0], None, &mut state, program, true);
	}
}

fn execute_tokens(current: &Token, next: Option<&Token>, state: &mut ParserState, program: &mut Program, single_token: bool) {
	match &state.status {
		ParseStateStatus::Top => {
			match current {
				Token::Keyphrase(kp) => {
					state.status = ParseStateStatus::Keyphrase(kp.clone());
					if single_token {
						execute_tokens(current, next, state, program, true);
					}
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
				/*Keyphrase::WriteEntry => {
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
				},*/
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
				/*Keyphrase::WithThe => {
					match current {
						Token::Operator(op) => {
							state.status = ParseStateStatus::Operation(op.clone());
						},
						_ => {
							sb_panic!(program.line_number);
						},
					}
				},*/
				/*Keyphrase::FromMemory => {
					if state.cached_keyphrase == Some(Keyphrase::Write) {
						//program.write_memory_value(state.cached_identifier.clone());
						state.cached_keyphrase = Some(Keyphrase::FromMemory);
						state.status = ParseStateStatus::Top;
					} else {
						sb_panic!(program.line_number);
					}
				},*/
				/*Keyphrase::FromDivineIntervention => {
					if state.cached_keyphrase == Some(Keyphrase::WriteEntry) {
						program.write_input_value(state.cached_identifier.clone());
						state.clear_cache();
					} else {
						sb_panic!(program.line_number);
					}
				},*/
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
				Keyphrase::PickUpChapterOffTheFloorAndPutItBack => {
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
					// FIXME: This is bad do this a better way please
					program.cast_cauldron_spell(&CauldronSpell::from_string(&state.cached_builtin).unwrap());
					state.clear_cache();
				},
				Keyphrase::KnockOverCauldron => {
					program.knock_over_cauldron();
					state.clear_cache();
				},
				Keyphrase::SkipTheNext => {
					match current {
						Token::Literal(lit) => {
							state.cached_literal = Some(lit.clone());
							state.cached_keyphrase = Some(Keyphrase::SkipTheNext);
							if next.is_none() {
								program.change_line_by(&lit);
							}
						},
						_ => {
							sb_panic!(program.line_number);
						}
					}
				},
				Keyphrase::PublishSpellbook => {
					program.publish(false, String::new());
					state.clear_cache();
				},
				/*Keyphrase::PublishSpellbookTo => {
					match current {
						Token::Builtin(bt) => {
							if bt == "console" {
								state.cached_builtin = bt.to_string();
							} else {
								sb_panic!(program.line_number);
							}
						},
						Token::Literal(lit) => {
							state.cached_literal = Some(lit.clone());
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
							} else if state.cached_literal.is_some() {
								match state.cached_literal.as_ref().unwrap() {
									Variant::Str(string) => {
										string.clone()
									},
									_ => {
										sb_panic!(program.line_number);
									},
								}
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
				},*/
				/*Keyphrase::WrappedUpWith => {
					match current {
						Token::Literal(lit) => {
							match lit {
								Variant::Str(st) => {
									let not_console = &state.cached_builtin != "console";
									program.publish(
										not_console,
										if !not_console {
											String::new()
										} else if state.cached_literal.is_some() {
											match state.cached_literal.as_ref().unwrap() {
												Variant::Str(string) => {
													string.clone()
												},
												_ => {
													sb_panic!(program.line_number);
												},
											}
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
				},*/
				/*Keyphrase::SignChapterWith => {
					match current {
						Token::Literal(lit) => {
							match lit {
								Variant::Str(string) => {
									program.
								},
								_ => {
									sb_panic!(program.line_number);
								},
							}
						},
						_ => {
							sb_panic!(program.line_number);
						}
					}
				},*/
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
				Keyphrase::ThrowSpellbookInTheTrash => {
					program.exit = true;
					state.clear_cache();
				},
				_ => {},
			}
		},
		
		/*ParseStateStatus::Operation(operator) => {
			let operand = match current {
				Token::Identifier(ident) => {
					match program.try_get_value(&ident) {
						Some(val) => val,
						None => {
							sb_panic!(program.line_number);
						},
					}
				},
				Token::Literal(lit) => {
					lit.clone()
				},
				Token::Operator(op) => {
					match op {
						Operator::And => {
							return;
						},
						_ => {
							sb_panic!(program.line_number);
						},
					}
				},
				_ => {
					sb_panic!(program.line_number);
				}
			};

			state.cached_operand_list.push(operand);

			if next.is_none() {
				match state.perform_operation(&operator) {
					Some(result) => {
						program.write_literal_value(state.cached_identifier.clone(), Some(result));
						state.clear_cache();
					},
					None => {
						sb_panic!(program.line_number);
					}
				}
			}
		},*/
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

	/*assert_eq!(tokenize_line("write entry hello with value 5".into()), Some(vec![
		T::Keyphrase(K::WriteEntry),
		T::Identifier("hello".into()),
		T::Keyphrase(K::WithValue),
		T::Literal(Variant::Integer(5)),
	]));*/
}

/* 
#[test]
fn test_operations() {
	type V = Variant;

	let mut state = ParserState::new();
	state.cached_operand_list = vec![V::Integer(5), V::Integer(3)];
	assert_eq!(state.perform_operation(&Operator::Sum), Some(V::Integer(8)));
	state.cached_operand_list = vec![V::Integer(5), V::Integer(3), V::Integer(17)];
	assert_eq!(state.perform_operation(&Operator::Sum), Some(V::Integer(25)));
	state.cached_operand_list = vec![V::Integer(5), V::Integer(3), V::Float(12.7)];
	assert_eq!(state.perform_operation(&Operator::Sum), None);

	state.cached_operand_list = vec![V::Integer(32), V::Integer(8), V::Integer(8)];
	assert_eq!(state.perform_operation(&Operator::Difference), Some(V::Integer(16)));
	state.cached_operand_list = vec![V::Integer(5), V::Integer(60)];
	assert_eq!(state.perform_operation(&Operator::Difference), Some(V::Integer(-55)));

	state.cached_operand_list = vec![V::Str("Hello ".into()), V::Str("there".into())];
	assert_eq!(state.perform_operation(&Operator::Concatenation), Some(V::Str("Hello there".into())));
}
*/
