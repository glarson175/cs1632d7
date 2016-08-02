extern crate rand;
use rand::Rng;
use std::io;

/// Statistics Struct that contains:
/// the total number of rounds that are played
/// the number of user wins, losses, and ties
/// the percentages of user wins, losses, and ties
/// times throwing rock, paper, and scissors

struct Statistics{
	total_plays: f64,
	user_wins: f64,
	percentage_wins: f64,
	user_ties: f64,
	percentage_ties: f64,
	user_losses: f64,
	percentage_losses: f64,
	rocks: i32,
	papers: i32,
	scissors: i32
} 

/// Enum that contains all valid options for the user and computer choice
/// Added 'NoEntry' to initialize my values. should never be changed to NoEntry for duration of the program

#[derive(Debug)]
enum Choice{
	Rock, 
	Paper,
	Scissors,
	NoEntry,
}

/// This Function is called at the beginning of every round, it will be used to check that the user has entered a valid/desired input. 
/// If the user does not enter a 'q','r', 'p', or 's' then this will loop until one of those is entered. this will not 
/// add anything to the total_plays value of the Struct

fn get_valid_entry()-> String{
	let mut choice = String::new();
	let mut valid_entry = false;
	while !valid_entry {
		choice = String::new();
		io::stdin().read_line(&mut choice).expect("you suck at programming");
		let choice = choice.trim();

		if choice =="q" || choice == "r" || choice == "p" || choice == "s" {
			valid_entry = true;
		} else {
			println!("Invalid Entry Try again, Enter (r,p,s) or q to quit");
			valid_entry = false;
		}
	}
	return choice;
}

/// This Function will be called at the end of the program when the user presses 'q' to quit. It takes a Statistics Struct as an argument 
/// it will make the calculation for WIN Percentage, LOSS Percentage, and TIE percentage
/// and print out all the information with in the Struct in the correct format

fn print_stats(mut print_struct: Statistics){
	if print_struct.total_plays == 0.0 {
		print_struct.percentage_wins = 0.0;
		print_struct.percentage_ties = 0.0;
		print_struct.percentage_losses = 0.0;
	}
	else {
		print_struct.percentage_wins = (print_struct.user_wins/print_struct.total_plays)*100.0;
		print_struct.percentage_ties = (print_struct.user_ties/print_struct.total_plays)*100.0;
		print_struct.percentage_losses = (print_struct.user_losses/print_struct.total_plays)*100.0;
	}
	println!("Player Stats:");
	println!("Wins: {} ({}%)", print_struct.user_wins, format!("{:.*}", 2, print_struct.percentage_wins));
	println!("Ties: {} ({}%)", print_struct.user_ties, format!("{:.*}", 2, print_struct.percentage_ties));
	println!("Losses: {} ({}%)", print_struct.user_losses, format!("{:.*}", 2, print_struct.percentage_losses));
	println!("Rocks: {}", print_struct.rocks);
	println!("Papers: {}", print_struct.papers);
	println!("Scissors: {}", print_struct.scissors);
}	

/// This Funtion creates a random number that is either 1, 2, or 3. and associates a Choice Enum Value to each number. 
/// Then returns the Choice that was decided on based on the random number.

fn get_computer_choice() -> Choice {
	let computer_choice_int = rand::thread_rng().gen_range(1,4);
	let mut computer_choice: Choice = Choice::NoEntry;
	if computer_choice_int == 1 {
		computer_choice = Choice::Rock;
	}
	else if computer_choice_int == 2 {
		computer_choice = Choice::Paper;
	}
	else if computer_choice_int == 3 {
		computer_choice = Choice::Scissors;
	}
	return computer_choice;
}

/// This Function takes the User Choice, the Computer Choice, and Borrows the Statistics struction passed in. 
/// It uses nested Matching statements to determine if the user has won, lost, or has tied. 
/// with each associated Match, it increases the user wins, losses, or ties accordingly. 

fn who_wins(user_val:Choice, computer_val:Choice, stat_to_change: &mut Statistics) {
	match user_val {
		Choice::Rock => {
			match computer_val{
				Choice::Rock => {
					stat_to_change.user_ties +=1.0; 
					println!("It's a Tie!");
					},
				Choice::Paper => {
					stat_to_change.user_losses +=1.0; 
					println!("You Lose!");
					},
				Choice::Scissors => {
					stat_to_change.user_wins +=1.0; 
					println!("You Win!");
					},
				_ => {}
			}
		},
		Choice::Paper => {
			match computer_val {
				Choice::Rock => {
					stat_to_change.user_wins +=1.0; 
					println!("You Win!");
				},
				Choice::Paper => {
					stat_to_change.user_ties +=1.0;
					println!("It's a Tie!");
				},
				Choice::Scissors => {
					stat_to_change.user_losses +=1.0;
					println!("You Lose!");
				},
				_ => {}
			}
		},
		Choice::Scissors => {
			match computer_val {
				Choice::Rock => {
					stat_to_change.user_losses +=1.0;
					println!("You Lose!");
				},
				Choice::Paper => {
					stat_to_change.user_wins +=1.0; 
					println!("You Win!");
				},
				Choice::Scissors => {
					stat_to_change.user_ties +=1.0;
					println!("It's a Tie!");
				},
				_ => {}
			}
		},
		_ => {}
	}
}

/// This Function Borrows a Statistics Struct and passes in a signed 32 bit int
/// it then adds the int argument to the Rocks value of the Struct

fn add_rocks(stat_to_change: &mut Statistics, value: i32) {
   stat_to_change.rocks += value;
}

/// This Function Borrows a Statistics Struct and passes in a signed 32 bit int
/// it then adds the int argument to the Papers value of the Struct

fn add_paper(stat_to_change: &mut Statistics, value: i32) {
   stat_to_change.papers += value;
}

/// This Function Borrows a Statistics Struct and passes in a signed 32 bit int
/// it then adds the int argument to the Scissors value of the Struct

fn add_scissors(stat_to_change: &mut Statistics, value: i32) {
   stat_to_change.scissors += value;
}

/// This Function Borrows a Statistics Struct and passes in a floating point 64 bit value
/// it then adds the int argument to the Rocks value of the Struct
/// (Made this a floating point because the total_plays value is a f64 and the types had to match)

fn change_total_plays(stat_to_change: &mut Statistics, value: f64) {
   stat_to_change.total_plays += value;
}
	
fn main() {
	let mut done = false; // mut done: bool used to play till user enters q.
	// stats_struct used for the duration of the program 
	let mut stats_struct: Statistics = Statistics{total_plays: 0.0, user_wins: 0.0, percentage_wins: 0.00, user_ties: 0.0, percentage_ties: 0.00, user_losses: 0.0, percentage_losses: 0.00, rocks: 0, papers: 0, scissors: 0};
	while !done {
		println!("Enter a Choice: (r,p,s) or q to quit");
	    let choice = get_valid_entry(); //gets a valid entry from the user
		let mut user_choice: Choice = Choice::NoEntry; // creates a Choice Enum for the User Choice and initializes it to NoEntry
		let computer_choice: Choice = get_computer_choice(); // creates a Choice Enum for the Computer Choice and sets it to the value of the get_computer_choice() function.
		let choice = choice.trim(); // jsut in case the value has any un needed white spaces
		//if/else if/else to determine user choice based on letter entered.
		if choice == "q" {
			done = true;
			change_total_plays(&mut stats_struct, -1.0);
			println!("");
		}
		else if choice == "r" {
			user_choice = Choice::Rock;
			add_rocks(&mut stats_struct, 1);
			println!("Player Chose: {:?}", user_choice);
			println!("Oponent Chose: {:?}", computer_choice);
		}
		else if choice == "p" {
			user_choice = Choice::Paper;
			add_paper(&mut stats_struct, 1);
			println!("Player Chose: {:?}", user_choice);
			println!("Oponent Chose: {:?}", computer_choice);
		}
		else if choice == "s" {
			user_choice = Choice::Scissors;
			add_scissors(&mut stats_struct, 1);
			println!("Player Chose: {:?}", user_choice);
			println!("Oponent Chose: {:?}", computer_choice);
		}
		else {
			println!("That is not a valid input, please enter either r, p, s, or q");
			change_total_plays(&mut stats_struct, -1.0);
		}
		change_total_plays(&mut stats_struct, 1.0);
		// sends User Choice Enum, Computer Choice Enum, and Statistics Struct to determine who won and update values
		who_wins(user_choice, computer_choice, &mut stats_struct); 
	}//end while !done
	
	print_stats(stats_struct);
}