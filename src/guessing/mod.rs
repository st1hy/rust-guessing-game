extern crate rand;

use std::cmp::Ordering;
use rand::{Rng};
use rand::distributions::{IndependentSample, Range};

pub mod random;
pub mod no_repetition_random;
pub mod prediction_random;
pub mod prediction;

pub type Guess = u8;
pub const MIN: Guess = 0;
pub const MAX: Guess = 100;


pub trait GuessingMethod {
	fn new_guess(&mut self, previous_result: &Option<Ordering>) -> Guess;
	fn reset(&mut self);
}

pub fn guess<R: Rng>(rng : &mut R) -> Guess {
	guess2(rng, &Range::new(MIN, MAX))
}

fn guess2<R: Rng>(rng : &mut R, range: &Range<Guess>) -> Guess {
	range.ind_sample(rng)
}
