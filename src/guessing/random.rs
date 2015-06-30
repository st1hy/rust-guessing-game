extern crate rand;

use std::cmp::Ordering;

use guessing;
use guessing::GuessingMethod;

type Guess = guessing::Guess;

pub struct TrulyRandom;

impl TrulyRandom {
	pub fn new() -> TrulyRandom {
		TrulyRandom
	}
}

impl guessing::GuessingMethod for TrulyRandom {
	fn new_guess(&mut self, _: &Option<Ordering>) -> Guess {
		guessing::guess()
	}
	fn reset(&mut self) {}

	fn clone(&self) -> Box<guessing::GuessingMethod> {
		Box::new(TrulyRandom)
	}
}

#[test]
fn it_works() {
	let mut g = TrulyRandom::new();
	let guess_result: Option<Ordering> = None;
	g.new_guess(&guess_result);
}
