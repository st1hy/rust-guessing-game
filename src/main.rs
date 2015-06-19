extern crate time;
extern crate rand;

mod guessing;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::boxed::Box;
// use std::thread;
// use std::sync::{Arc, Mutex};

type GuessingMethod = guessing::GuessingMethod;
type Guess = guessing::Guess;
type TrulyRandom = guessing::random::TrulyRandom;
type NoRepetitionRandom = guessing::no_repetition_random::NoRepetitionRandom;
type PredictionRandom = guessing::prediction_random::PredictionRandom;
type Prediction = guessing::prediction::Prediction;

const TEST_COUNT: u32 = 10_000;

struct GuessingMethodSimulation {
	name: &'static str,
	gm: Box<GuessingMethod>,
}

impl GuessingMethodSimulation{
	pub fn new(name: &'static str, method: Box<GuessingMethod>) -> GuessingMethodSimulation {
		GuessingMethodSimulation {
			name: name,
			gm: method
		}
	}

	fn simulate(&mut self) -> f64 {
		let mut result: f64 = 0.;
		let count = TEST_COUNT;
		let mut rand = rand::thread_rng();
		for _ in 0..count {
			self.gm.reset();
			let r = self.comp_guess(&mut rand);
			result+= r as f64;
		}
		result /= count as f64;
		result
	}

	fn comp_guess<R: Rng>(&mut self, rng : &mut R) -> u64 {
		let target : Guess = guessing::guess(rng);
		let mut number_of_tries : u64= 0;
		let mut guess: Guess;
		let mut guess_result: Option<Ordering> = None;
		loop {
			guess = self.gm.new_guess(&guess_result);
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

	fn print_simulation(&mut self) {
		print!("{} finds result on avarage", self.name);
		let past = time::precise_time_s();
		print!(" in {} tries",self.simulate());
		println!(" in {} s",time::precise_time_s() - past);
	}

}

struct SimulationEnv {
	simulations: Vec<GuessingMethodSimulation>
}

impl SimulationEnv {
	fn new() -> SimulationEnv {
		let mut simulation_vec = Vec::new();
		let s = GuessingMethodSimulation::new(
				"Random guessing with repetition",
				Box::new(TrulyRandom::new())
			);
		simulation_vec.push(s);
		let s = GuessingMethodSimulation::new(
				"Random guessing without repetition",
				Box::new(NoRepetitionRandom::new())
			);
		simulation_vec.push(s);
		let s = GuessingMethodSimulation::new(
				"Random guessing with prediction",
				Box::new(PredictionRandom::new())
			);
		simulation_vec.push(s);
		let s = GuessingMethodSimulation::new(
				"Prediction guessing",
				Box::new(Prediction::new())
			);
		simulation_vec.push(s);
		SimulationEnv{simulations: simulation_vec}
	}

	fn print_simulations(&mut self) {
		for s in self.simulations.iter_mut() {
			s.print_simulation();
		}
	}

	// fn print_simulations_parallel(&mut self) {
	// 	for s in self.simulations.iter_mut() {
	// 		let data = Arc::new(Mutex::new(8));
	// 		let d = data.clone();
	// 		thread::scoped(|| {
	// 				let sc = d.lock().unwrap();
	// 				println!("{}",sc.deref())
	// 				//sc.print_simulation();
	// 			});
	// 	}
	// }
}

fn main() {
	let min = guessing::MIN;
	let max = guessing::MAX;
	println!("Guessing game!");
	println!("Guessing value in range of {}..{} simulation overhead: {}",min, max, TEST_COUNT);
	SimulationEnv::new().print_simulations();
	user_guess();
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
