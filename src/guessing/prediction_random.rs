extern crate rand;

use std::cmp::Ordering;

use guessing;
use guessing::GuessingMethod;

pub type Guess = guessing::Guess;

pub struct PredictionRandom {
	rand: rand::ThreadRng,
    min: Guess,
    max: Guess,
    last: Guess,
}

impl PredictionRandom {
	pub fn new() -> PredictionRandom {
        PredictionRandom {
			rand: rand::thread_rng(),
            min: guessing::MIN,
            max: guessing::MAX,
            last: 0,
		}
	}

	pub fn reset(&mut self) {
        self.min = guessing::MIN;
        self.max = guessing::MAX;
        self.last = 0;
	}
}

impl guessing::GuessingMethod for PredictionRandom {

	fn new_guess(&mut self, previous_result: &Option<Ordering>) -> Guess {
        match *previous_result {
            Some(Ordering::Less) => self.max = self.last - 1,
            Some(Ordering::Greater) => self.min = self.last +1,
            _ => (),
        }
        if self.min == self.max {return self.min;}
        self.last = guessing::guess2(&mut self.rand, self.min, self.max);
        self.last
	}
	fn reset(&mut self) {
        PredictionRandom::reset(self);
	}
}

#[test]
fn it_works() {
	let mut g = PredictionRandom::new();
	let guess_result: Option<Ordering> = None;
	let result = g.new_guess(&guess_result);
}
