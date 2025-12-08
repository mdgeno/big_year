use crate::backend::database::BirdDatabase;
use std::io::{self, Write};

mod backend;
mod game;

fn main(){

	let mut birds = BirdDatabase::new_list();

	print!("? ");
	io::stdout().flush().unwrap();

	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("enter correct input");

	match input.trim(){
		"Add"         => birds.add(),
/*
		"Observation" =>
		"All"         =>
		"One"         =>
		"Quit"        =>
*/              _             => println!("enter correct command")
	}
	
	
/*
	let bird1 = Bird{ name: String::from("bird1_name"),
			  latin_name: String::from("bird1_latin_name"),
			  sightings: 0
			};
	let bird2 = Bird{ name: String::from("bird2_name"),
			  latin_name: String::from("bird2_latin_name"),
			  sightings: 0
			};


	birds.add(bird1);
	birds.add(bird2);
	
	birds.all();

	println!(" ");

	birds.spotted(&String::from("bird2_name"));
	birds.spotted(&String::from("bird2_name"));
	birds.all();

	birds.spotted(&String::from("random"));	

	println!(" ");
	
	birds.view(&String::from("bird2_name"));
*/
}
	