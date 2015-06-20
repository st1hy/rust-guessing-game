extern crate rand;

use std::cmp::Ordering;
use rand::distributions::{IndependentSample, Range};

pub mod random;
pub mod no_repetition_random;
pub mod prediction_random;
pub mod prediction;

pub type Guess = usize;
pub const MIN: Guess = 0;
pub const MAX: Guess = 100;


pub trait GuessingMethod: Send + Sync {
	fn new_guess(&mut self, previous_result: &Option<Ordering>) -> Guess;
	fn reset(&mut self);
}

pub fn guess() -> Guess {
	guess2(&Range::new(MIN, MAX))
}

fn guess2(range: &Range<Guess>) -> Guess {
	let mut rng = rand::thread_rng();
	range.ind_sample(&mut rng)
}
