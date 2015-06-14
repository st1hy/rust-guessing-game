extern crate rand;

use std::cmp::Ordering;
use std::vec::Vec;

use guessing;
use guessing::GuessingMethod;

pub type Guess = guessing::Guess;

pub struct NoRepetitionRandom {
	rand: rand::ThreadRng,
	last_guesses: Vec<Guess>,
}

impl NoRepetitionRandom {
	pub fn new() -> NoRepetitionRandom {
		NoRepetitionRandom {
			rand: rand::thread_rng(),
			last_guesses: Vec::new()
		}
	}

	pub fn reset(&mut self) {
		self.last_guesses.clear();
	}

	fn push(&mut self, g: Guess) {
		self.last_guesses.push(g);
		self.last_guesses.sort();
	}
}

impl guessing::GuessingMethod for NoRepetitionRandom {

	fn new_guess(&mut self, _: &Option<Ordering>) -> Guess {
		let max = guessing::MAX - self.last_guesses.len() as Guess;
		if max < 1 { panic!("No more guesses can be made.")}
		let mut g = guessing::guess2(&mut self.rand, 0, max);
		for i in &self.last_guesses {
			if i <= &g { g+=1 };
		}
		self.push(g);
		g
	}
	fn reset(&mut self) {
		NoRepetitionRandom::reset(self);
	}
}

#[test]
fn it_works() {
	let mut g = NoRepetitionRandom::new();
	let guess_result: Option<Ordering> = None;
	let result = g.new_guess(&guess_result);
}
