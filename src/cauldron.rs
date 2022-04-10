// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*
//        SPELLBOOK INTERPRETER
//           BY DIANE SPARKS
// *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*

use crate::variant::Variant;
use crate::page::*;
use std::io;

#[derive(Clone, PartialEq)]
pub enum CauldronSpell {
	Coadjuvancy,
	Judgement,
	Reverberate,

	Entwinement,
	Belittlement,
	Reenactment,
	Apportionment,

	Antipodize,
	Juxtapose,
	
	Amplify,
	Squelch,
	Diminish,

	Vacation,
}

pub enum CauldronSpellResult {
	DoNothing,
	NoCharge,
	SkipLine(usize),
	JumpBack(usize),
}

pub enum CauldronMixMode {
	Add,
	Sub,
	Mul,
	Div,
}

pub struct Cauldron {
	page: Option<Page>,

	spell_charge: usize,
	spell_charge_amplifier: usize,
	consecutive_amplifies: usize,

	mix_mode: CauldronMixMode,
}

impl Cauldron {
	pub fn new() -> Self {
		Self{
			page: None,

			spell_charge: 0,
			spell_charge_amplifier: 1,
			consecutive_amplifies: 0,

			mix_mode: CauldronMixMode::Add,
		}
	}

	pub fn get_amplifier(&self) -> usize {
		self.spell_charge_amplifier
	}

	pub fn increase_charge(&mut self, override_amount: bool, amount: usize) {
		self.spell_charge += if override_amount { amount } else { self.spell_charge_amplifier };
	}

	pub fn decrease_charge(&mut self, override_amount: bool, amount: usize) {
		self.spell_charge = self.spell_charge.saturating_sub(if override_amount { amount } else { self.spell_charge_amplifier });
	}

	pub fn reset_charge(&mut self) {
		self.spell_charge = 0;
		
	}

	pub fn reset_amplifier(&mut self) {
		self.spell_charge_amplifier = 1;
	}

	pub fn add_page(&mut self, page: &Page) -> bool {
		match self.page {
			Some(ref mut my_page) => {
				for i in 0..3 {
					if page.values[i].is_some() {
						my_page.entry_names[i].push_str(&page.entry_names[i]);
						match my_page.values[i] {
							Some(ref current_val) => {
								let result = match self.mix_mode {
									CauldronMixMode::Add => current_val.add(page.values[i].as_ref().unwrap().clone()),
									CauldronMixMode::Sub => current_val.sub(page.values[i].as_ref().unwrap().clone()),
									CauldronMixMode::Mul => current_val.mul(page.values[i].as_ref().unwrap().clone()),
									CauldronMixMode::Div => current_val.div(page.values[i].as_ref().unwrap().clone()),
								};

								match result {
									Some(success) => {
										my_page.values[i] = Some(success);
									},
									None => {
										return false;
									},
								}
							},
							None => {
								my_page.values[i] = page.values[i].clone();
							},
						}
					}
				}
			},
			None => {
				self.page = Some(page.clone());
			},
		}

		true
	}

	pub fn knock_over(&mut self) -> Option<Page> {
		let page = self.page.clone();
		self.page = None;
		page
	}

	pub fn cast_spell(&mut self, spell: &CauldronSpell) -> Option<CauldronSpellResult> {
		if *spell != CauldronSpell::Amplify && *spell != CauldronSpell::Vacation {
			self.consecutive_amplifies = 0;
		}
		
		match spell {
			CauldronSpell::Coadjuvancy => {
				match self.page {
					Some(ref mut pg) => {
						if pg.is_full() {
							return None;
						}

						loop {
							let mut input = String::with_capacity(10);
							if io::stdin().read_line(&mut input).is_err() {
								return None;
							}

							let input_trimmed = input.trim_end(); 
				
							let var = if let Ok(b) = input_trimmed.parse::<bool>() {
								Variant::Boolean(b)
							} else if let Ok(i) = input_trimmed.parse::<i64>() {
								Variant::Integer(i)
							} else if let Ok(f) = input_trimmed.parse::<f64>() {
								Variant::Float(f)
							} else {
								Variant::Str(input_trimmed.to_string())
							};
				
							if pg.write_value(String::new(), Some(var), false, 0) {
								return Some(CauldronSpellResult::DoNothing);
							}
						}
					},
					None => None,
				}
			},

			CauldronSpell::Entwinement => {
				self.mix_mode = CauldronMixMode::Add;
				Some(CauldronSpellResult::DoNothing)
			},
			CauldronSpell::Belittlement => {
				self.mix_mode = CauldronMixMode::Sub;
				Some(CauldronSpellResult::DoNothing)
			},
			CauldronSpell::Reenactment => {
				self.mix_mode = CauldronMixMode::Mul;
				Some(CauldronSpellResult::DoNothing)
			},
			CauldronSpell::Apportionment => {
				self.mix_mode = CauldronMixMode::Div;
				Some(CauldronSpellResult::DoNothing)
			},

			CauldronSpell::Antipodize => {
				match self.page {
					Some(ref mut pg) => {
						for i in 0..3 {
							pg.entry_names[i] = pg.entry_names[i].chars().rev().collect();
							pg.values[i] = match &pg.values[i] {
								Some(val) => val.inverted(),
								None => None,
							}
						}

						Some(CauldronSpellResult::DoNothing)
					},
					None => None,
				}
			},
			CauldronSpell::Juxtapose => {
				match self.page {
					Some(ref mut pg) => {
						let mut new_page = Page::new(PageType::Boolean);
						new_page.values[0] = Some(Variant::Boolean(pg.values[1].is_some() && pg.values[2].is_some()));
						new_page.values[1] = Some(Variant::Boolean(pg.values[0] >= pg.values[2]));
						new_page.values[2] = Some(Variant::Boolean(pg.values[0] == pg.values[1]));
						self.page = Some(new_page);
						
						Some(CauldronSpellResult::DoNothing)
					},
					None => None,
				}
			},
			CauldronSpell::Judgement => {
				match self.page {
					Some(ref pg) => {
						if pg.page_type == PageType::Boolean {
							if pg.values.iter().all(|b| match b {
								Some(b) => b.to_bool(),
								None => true,
							}) {
								Some(CauldronSpellResult::DoNothing)
							} else {
								Some(CauldronSpellResult::SkipLine(self.spell_charge))
							}
						} else {
							None
						}
					},
					None => None,
				}
			},

			CauldronSpell::Amplify => {
				self.spell_charge_amplifier += 1;
				self.consecutive_amplifies += 1;
				if self.consecutive_amplifies > 3 {
					None
				} else {
					Some(CauldronSpellResult::NoCharge)
				}
			},
			CauldronSpell::Squelch => {
				self.reset_charge();
				self.reset_amplifier();
				Some(CauldronSpellResult::NoCharge)
			},
			CauldronSpell::Diminish => {
				self.decrease_charge(true, 1);
				Some(CauldronSpellResult::NoCharge)
			},
			CauldronSpell::Reverberate => {
				Some(CauldronSpellResult::JumpBack(self.spell_charge))
			},

			CauldronSpell::Vacation => {
				Some(CauldronSpellResult::DoNothing)
			},
		}
	}
}
