use std::io;
//use rand::Rng;
use std::cmp::Ordering;
use std::time::{ SystemTime, Duration };
use std::thread::sleep;
use std::convert::TryInto;

fn main() {
    let mut player_alive: bool = true;
    let mut player_skill: u32 = 9;
    let mut player_score: u32 = 1;
    let mut zombie_count: u32 = 0;
    let mut zombies_killed: u32 = 0;

    println!("Welcome to Zombie War.\nPress [ENTER] to start.");

    let mut nothing = String::new();

	io::stdin().read_line(&mut nothing)
	    	.expect("Failure to Read Line!");

    // ask how many zombies
    println!("How many zombies do you wish to fight?");
    
    let zombie_count = get_integer();

    println!("Get ready to fight for your life!!");

    // main game loop
    while (player_alive && zombies_killed < zombie_count) {
        // create a random zombie
        let zombie_skill: u32 = create_zombie();

        // battle sequence
        if (zombie_skill > 10) {
            println!("\nHere comes a huge zombie! \t{} vs {}", zombie_skill, player_skill);
        }
        else {
            //cout << endl << "Here comes zombie " << zombie_Skilled + 1 << endl;
            println!("\nHere comes zombie {}\t {} vs {}", zombies_killed + 1, zombie_skill, player_skill);
        }

        println!("Fighting...");

        //let psk: i32 = (player_skill).try_into().unwrap();
        let skill_difference: i32 = {
        	let zombie_skill: i32 = (zombie_skill).try_into().unwrap();
        	let player_skill: i32 = (player_skill).try_into().unwrap();
        	player_skill - zombie_skill
        };

        if player_skill - zombie_skill > 0 {
            //println!("{}", 3000 - (player_skill - zombie_skill) * 100);
            if 3000 - (player_skill - zombie_skill) * 100 > 0 {
            	//sleep(Duration::new(2, 0));
                //Sleep(3000 - (player_skill - zombie_skill) * 100);
                sleep(Duration::new(0, 3000 - (player_skill - zombie_skill) * 100));
            } else {
                //Sleep(50);
                sleep(Duration::new(0, 50));
            }
        } else {
            println!("Fighting...");
            sleep(Duration::new(4, 0));
            println!("You're struggling...");
            sleep(Duration::new(4, 0));

            if rand() % 10 > 3 {
                println!("this isn't looking good...");
            } else {
                println!("\nZombie bit you, and you died.");
                player_alive = false;
                break;
            }

            sleep(Duration::new(4, 0));

            if rand() % 10 > 3 {
                println!("this is very bad...");
            } else {
                println!("\nZombie bit you, and you died.");
                player_alive = false;
                break;
            }

            sleep(Duration::new(4, 0));

            if rand() % 10 > 3 {
                println!("but you survived somehow.");
            } else {
                println!("\nZombie bit you, and you died.");
                player_alive = false;
                break;
            }            
        }

        {
            if (player_skill - zombie_skill > 7) {
                println!("You wasted the zombie!");
                player_score = player_score * 2;
            }

            else if (player_skill - zombie_skill > 5) {
                println!("You decapitated the zombie!");
                player_score = player_score * 2;
            }

            else if (player_skill - zombie_skill > 0) {
                println!("You killed the zombie!");
                player_score = player_score * 2;
            }

            else {
                println!("You killed the zombie, but suffered injuries.");
            }

            zombies_killed += 1;
            player_skill = 9 + zombies_killed / 3;
        }

        println!("");
        sleep(Duration::new(5, 0));
    }

    // end game
    if zombies_killed == zombie_count {
        // victory
        println!("You have survived the onslaught!");
    }
    else {
        // lost
        println!("You did not survive the zombie war.");
    }

    println!("Zombies killed: {}", zombies_killed);
    println!("Final score: {}", player_score);
}

fn nothingness() {
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