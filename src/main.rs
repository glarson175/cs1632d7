extern crate rand;
use rand::Rng;
use std::io;

//structure to store all my statistics for the game!
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

//Enum with the Choices that the user and computer can make
#[derive(Debug)]
enum Choice{
	Rock, 
	Paper,
	Scissors,
	NoEntry,
}

//This just makes sure that the option a user enters is valid, this will loop until they enter something correct
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

//this will calculate the percentage wins, percentage ties, and percentage losses. then print out all the information
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

//creates a random number and depending on what number 1-3 it returns Choice::rock, paper, or scissors for the computer choice
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

//Takes in the user choice, the computer choice, and the borrowed statistic structure to add to wins, losses, and ties. 
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

//Borrows the Statistics Struct to add to the rocks value
fn add_rocks(mut stat_to_change: &mut Statistics, value: i32) {
   stat_to_change.rocks += value;
}

//Borrows the Statistics Struct to add to the papers value
fn add_paper(mut stat_to_change: &mut Statistics, value: i32) {
   stat_to_change.papers += value;
}

//Borrows the Statistics Struct to add to the scissors value
fn add_scissors(mut stat_to_change: &mut Statistics, value: i32) {
   stat_to_change.scissors += value;
}

//Borrows the Statistics Struct to change the total_plays value
fn change_total_plays(mut stat_to_change: &mut Statistics, value: f64) {
   stat_to_change.total_plays += value;
}
	
fn main() {
	let mut done = false; // mut done: bool used to play till user enters q.
	let mut test_struct: Statistics = Statistics{total_plays: 0.0, user_wins: 0.0, percentage_wins: 0.00, user_ties: 0.0, percentage_ties: 0.00, user_losses: 0.0, percentage_losses: 0.00, rocks: 0, papers: 0, scissors: 0};
	while !done {
	
		println!("Enter a Choice: (r,p,s) or q to quit");
	    let choice = get_valid_entry(); //
		let mut user_choice: Choice = Choice::NoEntry;
		let computer_choice: Choice = get_computer_choice();
		let choice = choice.trim();
		if choice == "q" {
			done = true;
			change_total_plays(&mut test_struct, -1.0);
			println!("");
		}
		else if choice == "r" {
			user_choice = Choice::Rock;
			add_rocks(&mut test_struct, 1);
			println!("Player Chose: {:?}", user_choice);
			println!("Oponent Chose: {:?}", computer_choice);
		}
		else if choice == "p" {
			user_choice = Choice::Paper;
			add_paper(&mut test_struct, 1);
			println!("Player Chose: {:?}", user_choice);
			println!("Oponent Chose: {:?}", computer_choice);
		}
		else if choice == "s" {
			user_choice = Choice::Scissors;
			add_scissors(&mut test_struct, 1);
			println!("Player Chose: {:?}", user_choice);
			println!("Oponent Chose: {:?}", computer_choice);
		}
		else {
			println!("That is not a valid input, please enter either r, p, s, or q");
			change_total_plays(&mut test_struct, -1.0);
		}
		change_total_plays(&mut test_struct, 1.0);
		who_wins(user_choice, computer_choice, &mut test_struct);
	}
	print_stats(test_struct);
}