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
			println!("{} ({}): number of sightings {}", bird.name.trim(), bird.latin_name.trim(), bird.sightings);
		}
	}

	pub fn spotted(&mut self){
		let bird = Self::b_name();

		for b in &mut self.birds{
			if b.name == bird{
				b.sightings +=1;
				return;
			}
		}	
		println!("Not a bird!");
	}

	pub fn view(&self){
		let bird = Self::b_name();

		for b in &self.birds{
			if b.name == bird{
				println!("{} ({}): number of sightings {}", b.name.trim(), b.latin_name.trim(), b.sightings);
				break;
			}
		}
	}

	fn b_name() -> String{
		print!("Bird? ");
		io::stdout().flush().unwrap();

		let mut b_name = String::new();
		io::stdin().read_line(&mut b_name).expect("enter correct name");

		b_name
	}
}