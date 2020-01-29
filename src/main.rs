use std::io;
//use rand::Rng;
use std::cmp::Ordering;
use std::time::{ SystemTime, Duration };
use std::thread::sleep;
use std::convert::TryInto;

fn main() {

    println!("Welcome to Zombie War.\nPress [ENTER] to start.");

    let mut nothing = String::new();

	io::stdin().read_line(&mut nothing)
	    	.expect("Failure to Read Line!");

    // ask how many zombies
    println!("How many zombies do you wish to fight?");
    
    let zombie_count = get_integer();

    println!("Get ready to fight for your life!!");

    //cin >> zombieCount;


	// sleep(Duration::new(2, 0));

	//println!("Time: {:?}", seconds);
	//println!("Number: {}", seconds % 100);
    println!("Guess the number!");

    //let secret_number = rand:thread_rng().gen_range(1, 101);
    //let secret_number = 7;
    //let secret_number: u32 = (seconds % 100).try_into().unwrap();
    let secret_number: u32 = rand();

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

fn rand() -> u32 {
	let seconds: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
		Ok(n) => n.as_secs(),
		Err(_) => panic!("SystemTime is before Unix Epoch!?"),
	};

	(seconds % 100).try_into().unwrap()
}

fn create_zombie() -> u32 {
    if rand() % 67 < 10 {
        11
    } else {
        rand() % 10 + 1
    }
}

fn get_integer() -> u32 {
	let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    	.expect("Failure to Read Line!");

	let guess: u32 = match guess.trim().parse() {
		Ok(num) => num,
		Err(_) => panic!("That is not a number!"),
	};

	guess
}