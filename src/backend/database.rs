use crate::game::Bird;
use std::io::{self, Write};

pub struct BirdDatabase{
	birds: Vec<Bird>
}

impl BirdDatabase{
	pub fn new_list() -> BirdDatabase{
		Self{ birds: Vec::new() }
	}

	pub fn add(&mut self){
		
		print!("Name: ");
		io::stdout().flush().unwrap();

		let mut b_name = String::new();
		io::stdin().read_line(&mut b_name).expect("enter correct input");
		
		
		print!("Name in Latin: ");
		io::stdout().flush().unwrap();

		let mut b_latin_name = String::new();
		io::stdin().read_line(&mut b_latin_name).expect("enter correct input");
		
		let bird = Bird{ name: b_name,
				 latin_name: b_latin_name,
				 sightings: 0
				};

		self.birds.push(bird);
	}

	pub fn all(&self){
		for bird in &self.birds{
			println!("{} ({}): number of sightings {}", bird.name, bird.latin_name, bird.sightings);
		}
	}

	pub fn spotted(&mut self, bird_name: &String){
		for b in &mut self.birds{
			if b.name == *bird_name{
				b.sightings +=1;
				return;
			}
		}	
		println!("not a bird");
	}

	pub fn view(&self, bird_name: &String){
		for b in &self.birds{
			if b.name == *bird_name{
				println!("{} ({}): number of sightings {}", b.name, b.latin_name, b.sightings);
				break;
			}
		}
	}
}