use crate::backend::database::BirdDatabase;
use std::io::{self, Write};

mod backend;
mod game;

fn main(){

	let mut birds = BirdDatabase::new_list();

	loop{
		print!("? ");
		io::stdout().flush().unwrap();

		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("enter correct input");

		match input.trim(){
			"Add"         => birds.add(),
			"All"         => birds.all(),

			"Observation" => birds.spotted(),
			"One"         => birds.view(),
			"Quit"        => break,
            	 	_             => println!("enter correct command")
		}
	}

}
	