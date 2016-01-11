extern crate rand;

use std::cmp::Ordering;
use rand::distributions::{IndependentSample, Range};

pub mod random;
pub mod iterate;
pub mod prediction_random;
pub mod prediction;

pub type Guess = usize;
pub const MIN: Guess = 0;
pub const MAX: Guess = 1000;


pub trait GuessingMethod: Send + Sync {
	fn new_guess(&mut self, previous_result: &Option<Ordering>) -> Guess;
	fn reset(&mut self);
	fn clone(&self) -> Box<GuessingMethod>;
}

pub fn guess() -> Guess {
	guess2(&Range::new(MIN, MAX))
}

fn guess2(range: &Range<Guess>) -> Guess {
	let mut rng = rand::thread_rng();
	range.ind_sample(&mut rng)
}
