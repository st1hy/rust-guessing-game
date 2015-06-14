extern crate time;
extern crate rand;

mod guessing;

use std::io;
use std::cmp::Ordering;

type GuessingMethod = guessing::GuessingMethod;
type Guess = guessing::Guess;
type TrulyRandom = guessing::random::TrulyRandom;
type NoRepetitionRandom = guessing::no_repetition_random::NoRepetitionRandom;
type PredictionRandom = guessing::prediction_random::PredictionRandom;
type Prediction = guessing::prediction::Prediction;

const TEST_COUNT: u32 = 10_000;

fn main() {
	let min = guessing::MIN;
	let max = guessing::MAX;
	println!("Guessing game!");
	println!("Guessing value in range of {}..{} simulation overhead: {}",min, max, TEST_COUNT);
	print_simulation(&mut TrulyRandom::new(), "Random guessing with repetition");
	print_simulation(&mut NoRepetitionRandom::new(), "Random guessing without repetition");
	print_simulation(&mut PredictionRandom::new(), "Random guessing with prediction");
	print_simulation(&mut Prediction::new(), "Prediction guessing");
	user_guess();
}


fn print_simulation(method: &mut GuessingMethod, method_name: &str) {
	print!("{} finds result on avarage", method_name);
	let past = time::precise_time_s();
	print!(" in {} tries",simulation(method));
	println!(" in {} s",time::precise_time_s() - past);
}

fn simulation(method: &mut GuessingMethod) -> f64 {
	let mut result: f64 = 0.;
	let count = TEST_COUNT;
	let mut rand = rand::thread_rng();
	for _ in 0..count {
		method.reset();
		let r = comp_guess(&mut rand, method);
		result+= r as f64;
	}
	result /= count as f64;
	result
}

fn comp_guess(rng : &mut rand::ThreadRng, method: &mut GuessingMethod) -> u64 {
	let target : Guess = guessing::guess(rng);
	let mut number_of_tries : u64= 0;
	let mut guess: Guess;
	let mut guess_result: Option<Ordering> = None;
	loop {
		guess = method.new_guess(&guess_result);
		number_of_tries+=1;
		let compare_result = target.cmp(&guess);
		guess_result = Some(compare_result);
		match compare_result {
			Ordering::Equal => break,
			_ => continue,
		}
	}
	number_of_tries
}

fn user_guess() {
	println!("Your turn!");
	let y : Guess = guessing::guess(&mut rand::thread_rng());
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
