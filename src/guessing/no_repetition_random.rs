extern crate rand;

use std::cmp::Ordering;
use rand::distributions::{Range};

use guessing;
use guessing::GuessingMethod;

pub type Guess = guessing::Guess;

pub struct NoRepetitionRandom {
	has_been_tried: [bool; guessing::MAX],
	number_of_tries: Guess,
}

impl NoRepetitionRandom {
	pub fn new() -> NoRepetitionRandom {
		NoRepetitionRandom {
			has_been_tried: [false; guessing::MAX],
			number_of_tries: 0,
		}
	}

	pub fn reset(&mut self) {
		self.has_been_tried = [false; guessing::MAX];
		self.number_of_tries = 0;
	}
}

impl guessing::GuessingMethod for NoRepetitionRandom {

	fn new_guess(&mut self, _: &Option<Ordering>) -> Guess {
		let max = guessing::MAX - self.number_of_tries as Guess;
		if max < 1 { panic!("No more guesses can be made.")}
		let mut g = guessing::guess2(&Range::new(0,max));
		for i in g..self.has_been_tried.len() {
			if i <= g && self.has_been_tried[i] { g+=1 };
		}
		self.number_of_tries+=1;
		self.has_been_tried[g] = true;
		g
	}
	fn reset(&mut self) {
		NoRepetitionRandom::reset(self);
	}
	fn clone(&self) -> Box<guessing::GuessingMethod> {
		let mut array_clone = [false; guessing::MAX];
		for i in 0..self.has_been_tried.len() {
			array_clone[i] = self.has_been_tried[i];
		}
		let a = NoRepetitionRandom {
			has_been_tried: array_clone,
			number_of_tries: self.number_of_tries
		};
		Box::new(a)

	}
}

#[test]
fn it_works() {
	let mut g = NoRepetitionRandom::new();
	let guess_result: Option<Ordering> = None;
	let result = g.new_guess(&guess_result);
}
