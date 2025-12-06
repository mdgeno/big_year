use crate::backend::database::BirdDatabase;
use crate::game::Bird;

mod backend;
mod game;

fn main(){
	
	let mut birds = BirdDatabase::new_list();

	let bird1 = Bird{ name: String::from("bird1_name"),
			  latin_name: String::from("bird1_latin_name"),
			  sightings: 0
			};
	let bird2 = Bird{ name: String::from("bird2_name"),
			  latin_name: String::from("bird2_latin_name"),
			  sightings: 4
			};


	birds.add(bird1);
	birds.add(bird2);
	
	birds.list();	

}
	