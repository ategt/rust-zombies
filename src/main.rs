use std::io;
//use rand::Rng;

fn main() {
    println!("Guess the number!");

    //let secret_number = rand:thread_rng().gen_range(1, 101);
    let secret_number = 7;

    println!("The Secret Number is {}", secret_number);

    println!("Please Guess a Number: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    	.expect("Failure to Read Line!");

	println!("\nYou Guessed {}", guess);
}
