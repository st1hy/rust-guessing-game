extern crate rand;

use std::cmp::Ordering;

use guessing;
use guessing::GuessingMethod;

pub type Guess = guessing::Guess;

pub struct Prediction {
    min: Guess,
    max: Guess,
    last: Guess,
}

impl Prediction {
	pub fn new() -> Prediction {
        Prediction {
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

impl guessing::GuessingMethod for Prediction {

	fn new_guess(&mut self, previous_result: &Option<Ordering>) -> Guess {
        match *previous_result {
            Some(Ordering::Less) => self.max = self.last - 1,
            Some(Ordering::Greater) => self.min = self.last +1,
            _ => (),
        }
        if self.min == self.max {return self.min;}
        self.last = (self.max - self.min) / 2 + self.min;
        self.last
	}
	fn reset(&mut self) {
        Prediction::reset(self);
	}

	fn clone(&self) -> Box<guessing::GuessingMethod> {
		let a =Prediction {
            min: self.min,
            max: self.max,
            last: self.last,
		};
        Box::new(a)
	}
}

#[test]
fn it_works() {
	let mut g = Prediction::new();
	let guess_result: Option<Ordering> = None;
	let result = g.new_guess(&guess_result);
}
