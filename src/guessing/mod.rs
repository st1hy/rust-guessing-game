extern crate rand;

use std::cmp::Ordering;
use rand::Rng;

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

#[inline]
pub fn guess(rng : &mut rand::ThreadRng) -> Guess {
	guess2(rng, MIN, MAX)
}

#[inline]
fn guess2(rng : &mut rand::ThreadRng, min: Guess, max: Guess) -> Guess {
	rng.gen_range::<Guess>(min,max)
}
