use std::io;
//use rand::Rng;
use std::cmp::Ordering;
use std::time::{ SystemTime, Duration };
use std::thread::sleep;
use std::convert::TryInto;

fn main() {
	let st = SystemTime::now();

	// sleep(Duration::new(2, 0));

	// match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
	// 	Ok(n) => println!("1970 UTC was {} seconds ago.", n.as_secs()),
	// 	Err(_) => panic!("SystemTime is before Unix Epoch!?"),
	// }

	let seconds = match st.duration_since(SystemTime::UNIX_EPOCH) {
		Ok(n) => n.as_secs(),
		Err(_) => panic!("SystemTime is before Unix Epoch!?"),
	};

	//println!("Time: {:?}", seconds);
	//println!("Number: {}", seconds % 100);
    println!("Guess the number!");

    //let secret_number = rand:thread_rng().gen_range(1, 101);
    //let secret_number = 7;
    let secret_number: u32 = (seconds % 100).try_into().unwrap();

    println!("The Secret Number is {}", secret_number);

    loop {
    	
	    println!("Please Guess a Number: ");

	    let mut guess = String::new();

	    io::stdin().read_line(&mut guess)
	    	.expect("Failure to Read Line!");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
				//.expect("Please Type A Number!");

		println!("\nYou Guessed {}", guess);

		match guess.cmp(&secret_number) {
			Ordering::Less  => println!("Too Small!"),
			Ordering::Greater  => println!("Too Large!"),
			Ordering::Equal  => {
				println!("Equal");
				break;
			},
		}
	}
}
