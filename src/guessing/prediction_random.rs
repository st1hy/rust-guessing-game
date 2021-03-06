extern crate rand;

use std::cmp::Ordering;
use rand::distributions::{Range};

use guessing;
use guessing::GuessingMethod;

pub type Guess = guessing::Guess;

pub struct PredictionRandom {
    min: Guess,
    max: Guess,
    last: Guess,
}

impl PredictionRandom {
	pub fn new() -> PredictionRandom {
        PredictionRandom {
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
        self.last = guessing::guess2(&Range::new(self.min, self.max));
        self.last
	}
	fn reset(&mut self) {
        PredictionRandom::reset(self);
	}
	fn clone(&self) -> Box<guessing::GuessingMethod> {
		let a = PredictionRandom {
            min: self.min,
            max: self.max,
            last: self.last,
		};
        Box::new(a)
	}
}

#[test]
fn it_works() {
	let mut g = PredictionRandom::new();
	let guess_result: Option<Ordering> = None;
	let result = g.new_guess(&guess_result);
}
