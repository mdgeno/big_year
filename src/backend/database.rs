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

	pub fn list(&self){
		for bird in &self.birds{
			println!("{} ({}): number of sightings {}", bird.name, bird.latin_name, bird.sightings);
		}
	}
}