extern crate rand;

use std::cmp::Ordering;

use guessing;
use guessing::GuessingMethod;

type Guess = guessing::Guess;

pub struct TrulyRandom {
	rand: rand::ThreadRng,
}

impl TrulyRandom {
	pub fn new() -> TrulyRandom {
		TrulyRandom {rand: rand::thread_rng()}
	}

	//pub fn new_shared(rand2: rand::ThreadRng) -> TrulyRandom {
	//	TrulyRandom {rand: rand2}
	//}
}

impl guessing::GuessingMethod for TrulyRandom {
	fn new_guess(&mut self, _: &Option<Ordering>) -> Guess {
		guessing::guess(&mut self.rand)
	}
}


#[test]
fn it_works() {
	let mut g = TrulyRandom::new();
	let guess_result: Option<Ordering> = None;
	g.new_guess(&guess_result);
}