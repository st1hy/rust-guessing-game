extern crate rand;

use std::cmp::Ordering;
use rand::Rng;

pub mod random;
pub mod no_repetition_random;

pub type Guess = u8;
pub const MIN: Guess = 0;
pub const MAX: Guess = 100;


pub trait GuessingMethod {
	fn new_guess(&mut self, previous_result: &Option<Ordering>) -> Guess;
}

#[inline]
pub fn guess(rng : &mut rand::ThreadRng) -> Guess {
	rng.gen_range::<Guess>(MIN,MAX+1)
}

