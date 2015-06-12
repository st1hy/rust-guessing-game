extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

type Guess = u8;
const MIN: Guess = 0;
const MAX: Guess = 100;

fn main() {
	println!("Guessing game!");
	println!("Guessing value in range of {min}..{max}",min=MIN, max=MAX);
	println!("Random guessing with repetition finds result on avarage in {} tries",simulation(&mut TrulyRandom::new()));
	println!("Your turn!");
	user_guess();
}

trait GuessingMethod {
	fn new_guess(&mut self, previous_result: &Option<Ordering>) -> Guess;
}

struct TrulyRandom {
	rand: rand::ThreadRng,
}

impl TrulyRandom {
	fn new() -> TrulyRandom {
		TrulyRandom {rand: rand::thread_rng()}
	}
	
	fn new_shared(rand2: rand::ThreadRng) -> TrulyRandom {
		TrulyRandom {rand: rand2}
	}
}

impl GuessingMethod for TrulyRandom {
	fn new_guess(&mut self, _: &Option<Ordering>) -> Guess {
		guess(&mut self.rand)
	}
}

#[inline]
fn guess(rng : &mut rand::ThreadRng) -> Guess {
	rng.gen_range::<Guess>(MIN,MAX+1)
}

fn simulation(method: &mut GuessingMethod) -> f64 {
	let count = 10_000;
	let mut result: f64 = 0.;
	let mut rand = rand::thread_rng();
	for _ in 0..count {
		let r = comp_guess(&mut rand, method);
		//println!("{}", r);
		result+= r as f64;
	}
	result /= count as f64;
	result
}

fn comp_guess(rng : &mut rand::ThreadRng, method: &mut GuessingMethod) -> u64 {
	let target : Guess = guess(rng);
	let mut number_of_tries : u64= 0;
	let mut guess: Guess;
	let mut guess_result: Option<Ordering> = None;
	loop {
		guess = method.new_guess(&guess_result);
		number_of_tries+=1;
		let compare_result = guess.cmp(&target);
		guess_result = Some(compare_result);
		match compare_result {
			Ordering::Equal => break,
			_ => continue,
		}
	}
	number_of_tries
}

fn user_guess() {
	let y : Guess = guess(&mut rand::thread_rng());
	let mut number_of_tries = 0;
	loop {
		println!("Guess a number between 0 - 100: ");
		let mut x = String::new();
		io::stdin().read_line(&mut x)
			.ok()
			.expect("String is empty!");
		let x = x.trim().parse::<Guess>()
			.ok()
			.expect("Its not a number!");
		number_of_tries+=1;
		match x.cmp(&y) {
			Ordering::Less => println!("To small"),
			Ordering::Greater => println!("To big"),
			Ordering::Equal => { 
				println!("You won in {} tries!", number_of_tries);
				break;
			}
		}
	}
}
