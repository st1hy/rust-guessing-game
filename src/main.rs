extern crate time;
extern crate rand;

mod guessing;

use std::io;
use std::cmp::Ordering;
use std::boxed::Box;
use std::thread;
use std::sync::{Arc,Mutex};

type GuessingMethod = guessing::GuessingMethod;
type Guess = guessing::Guess;
type TrulyRandom = guessing::random::TrulyRandom;
type NoRepetitionRandom = guessing::no_repetition_random::NoRepetitionRandom;
type PredictionRandom = guessing::prediction_random::PredictionRandom;
type Prediction = guessing::prediction::Prediction;

const TEST_COUNT: usize = 10_000;

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
		for _ in 0..count {
			self.gm.reset();
			let r = self.comp_guess();
			result+= r as f64;
		}
		result /= count as f64;
		result
	}

	fn comp_guess(&mut self) -> u64 {
		let target : Guess = guessing::guess();
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
		let past = time::precise_time_s();
		let sim_result = self.simulate();
		let time = time::precise_time_s() - past;
		println!("{} finds result on avarage in {} tries in {} s", self.name, sim_result, time);
	}

}

struct SimulationEnv {
	simulations: Vec<Arc<Mutex<GuessingMethodSimulation>>>
}

impl SimulationEnv {
	fn new() -> SimulationEnv {
		let mut simulation_vec = Vec::new();
		let s = GuessingMethodSimulation::new(
				"Random guessing with repetition",
				Box::new(TrulyRandom::new())
			);
		simulation_vec.push(Arc::new(Mutex::new(s)));
		let s = GuessingMethodSimulation::new(
				"Random guessing without repetition",
				Box::new(NoRepetitionRandom::new())
			);
		simulation_vec.push(Arc::new(Mutex::new(s)));
		let s = GuessingMethodSimulation::new(
				"Random guessing with prediction",
				Box::new(PredictionRandom::new())
			);
		simulation_vec.push(Arc::new(Mutex::new(s)));
		let s = GuessingMethodSimulation::new(
				"Prediction guessing",
				Box::new(Prediction::new())
			);
		simulation_vec.push(Arc::new(Mutex::new(s)));

		SimulationEnv{simulations: simulation_vec}
	}


	fn print_simulations(&mut self) {
		for s in self.simulations.iter_mut() {
			let mut sim = s.lock().unwrap();
			sim.print_simulation();
		}
	}

	fn print_simulations_parallel(&mut self) {
		let v: Vec<_> = self.simulations.iter().map(|arc| {
			let mutex = arc.clone();
			thread::spawn(move || {
				let mut sim = mutex.lock().unwrap();
				sim.print_simulation();
			})
		}).collect();
		for thread in v.into_iter() {
			match thread.join() {
				Ok(_) => (),
				Err(e) => println!("Thread panic! {:?}",e),
			}
		};
	}
}

fn main() {
	let min = guessing::MIN;
	let max = guessing::MAX;
	println!("Guessing game!");
	println!("Guessing value in range of {}..{} simulation overhead: {}",min, max, TEST_COUNT);
	let mut env = SimulationEnv::new();
	env.print_simulations();
	println!("Now in parrallel!");
	env.print_simulations_parallel();
	user_guess();
}

fn user_guess() {
	println!("Your turn!");
	let y : Guess = guessing::guess();
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
