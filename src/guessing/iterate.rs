extern crate rand;

use std::cmp::Ordering;

use guessing;
use guessing::GuessingMethod;

pub type Guess = guessing::Guess;

pub struct IterateGuess {
	number_of_tries: Guess,
}

impl IterateGuess {
	pub fn new() -> IterateGuess {
		IterateGuess {
			number_of_tries: 0,
		}
	}

	pub fn reset(&mut self) {
		self.number_of_tries = 0;
	}
}

impl guessing::GuessingMethod for IterateGuess {

	fn new_guess(&mut self, _: &Option<Ordering>) -> Guess {
		let g = self.number_of_tries;
		self.number_of_tries+=1;
		g
	}
	fn reset(&mut self) {
		IterateGuess::reset(self);
	}
	fn clone(&self) -> Box<guessing::GuessingMethod> {
		let a = IterateGuess {
			number_of_tries: self.number_of_tries
		};
		Box::new(a)

	}
}

#[test]
fn it_works() {
	let mut g = IterateGuess::new();
	let guess_result: Option<Ordering> = None;
	let result = g.new_guess(&guess_result);
}
