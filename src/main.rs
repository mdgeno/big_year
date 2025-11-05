use std::io;

#[derive(Debug)]
#[derive(Clone)]
struct Bird{ 
	name: String,
	latin_name: String 
}

fn main(){
	
	let empty_bird = Bird{ name: String::from("empty"), latin_name: String::from("empty")};
	let mut array: [Bird; 5] = std::array::from_fn(|_| empty_bird.clone());
	let mut counter = 0;
	loop{
		println!("Enter command:");
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("enter correct input");
		println!(" ");

		match input.trim(){
			"add" => {array[counter] = add();
				  counter+=1;
				  println!(" ");},
		            _ => break
		}

		if counter == array.len()-1{
			let mut new_array: [Bird; array.len()*0.50] = std::array::from_fn(|_| empty_bird.clone());
			let mut i=0;
			for val in array{
				new_array[i] = val;
				i+=1;
			}
			let array = new_array;
		}	
	}	

	println!(" ");
	for b in array{
		println!("{} {}", b.name, b.latin_name);
	};
	println!("counter is at {counter}");	
}

fn add() -> Bird{
	println!("name: ");
	let mut n = String::new();
	io::stdin().read_line(&mut n).expect("enter correct input");
	
	println!("latin name: ");
	let mut ln = String::new();
	io::stdin().read_line(&mut ln).expect("enter correct input");
	
	Bird{ name: n.trim().to_string(), latin_name: ln.trim().to_string()}
}
	