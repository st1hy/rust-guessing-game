extern crate rand;

use std::cmp::Ordering;
use rand::Rng;
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
}

impl guessing::GuessingMethod for NoRepetitionRandom {

	fn new_guess(&mut self, previous_result: &Option<Ordering>) -> Guess {
		match *previous_result {
			Some(Ordering::Less) => {
				1 as Guess
			},
			Some(Ordering::Greater) => {
				2 as Guess
			},
			Some(Ordering::Equal) => {
				3 as Guess
			},
			None => {
				let g = guessing::guess(&mut self.rand);
				self.last_guesses.push(g);
				g
			},
		}
	}
}

pub fn guess(rng : &mut rand::ThreadRng, min: Guess, max: Guess) -> Guess {
	rng.gen_range::<Guess>(min,max+1)
}

#[test]
fn it_works() {
	let mut g = NoRepetitionRandom::new();
	let guess_result: Option<Ordering> = None;
	let result = g.new_guess(&guess_result);
	assert!(g.new_guess(&Some(Ordering::Less))==1);
	assert!(g.new_guess(&Some(Ordering::Greater))==2);
	assert!(g.new_guess(&Some(Ordering::Equal))==3);
	println!("{}",result);
}
