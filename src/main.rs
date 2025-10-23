//use std::io;

#[derive(Debug)]
struct Bird{ 
	name: String,
	latin_name: String 
}

fn main(){
	
	let bird_1 = Bird{
			name: String::from("1"),
			latin_name: String::from("11")
	};

	let bird_2 = Bird{
			name: String::from("2"),
			latin_name: String::from("22")
	};

	let bird_3 = Bird{
			name: String::from("3"),
			latin_name: String::from("33")
	};

	let bird_array: [Bird; 3] = [bird_1, bird_2, bird_3];

	println!("bird array contents {:?}", bird_array);

/*	
	print!("Name: ");
	let mut bird_name = String::new();
	io::stdin().read_line(&mut bird_name).expect("enter correct input");

	print!("Name in Latin: ");
	let mut latin_bird = String::new();
	io::stdin().read_line(&mut latin_bird).expect("enter correct input");
*/
		
}