use std::io;
//use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    //let secret_number = rand:thread_rng().gen_range(1, 101);
    let secret_number = 7;

    println!("The Secret Number is {}", secret_number);

    println!("Please Guess a Number: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    	.expect("Failure to Read Line!");

	let guess: u32 = guess.trim().parse()
			.expect("Please Type A Number!");

	println!("\nYou Guessed {}", guess);

	match guess.cmp(&secret_number) {
		Ordering::Less  => println!("Too Small!"),
		Ordering::Greater  => println!("Too Large!"),
		Ordering::Equal  => println!("Equal"),
	}
}
