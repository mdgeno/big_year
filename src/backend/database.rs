use crate::game::Bird;

pub struct BirdDatabase{
	birds: Vec<Bird>
}

impl BirdDatabase{
	pub fn new_list() -> BirdDatabase{
		Self{ birds: Vec::new() }
	}

	pub fn add(&mut self, bird: Bird){

		//when the program is refactored to be user interactive,
		//make sure to automatically set the sightings field of
		//the bird object to 0 initiatlly

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