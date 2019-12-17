//GradeTracker
//Author: Brian Jiang
//Date Created: 12/6/2019

//Commands: 
//addcourse [coursename]
//rmcourse [coursename] (if it exists)
//view [coursename]
//list
//gpa
//add [summative name][coursename][score (percentage or raw)][weight] 
//try [summative name][coursename][score(percentage or raw)][weight]
//hide [summative name][coursename]

use std::io; 

mod courses;
mod bash;
mod jsondata;
mod summative;   

fn main() { 
	let mut command = String::new(); 
	println!("Welcome to GradeTracker!\n"); 
	loop {
		command.clear(); 
		io::stdin().read_line(&mut command)
			.expect("Failed to read line");
		command = command.trim().to_string().to_ascii_uppercase(); 
		match command.as_ref() {
			"QUIT" => break,
			"EXIT" => break, 
			_ => process(&command), 
		}
	}
}

fn process(command: &String) { 
	match command.as_ref() {
	"ADDCOURSE" => match courses::addcourse() {
		Ok(()) => println!("\n"), 
		Err(why) => println!("Adding course was unsuccessful: {}\n", why) 
	},
	"RMCOURSE" => match courses::rmcourse() {
		Ok(()) => println!("\n"), 
		Err(why) => println!("Removing course was unsuccessful: {}\n", why) 
	},
	"VIEW" => match courses::view() {
		Ok(()) => println!("\n"), 
		Err(why) => println!("Could not view specified course: {}\n", why)
	},
	"LIST" => match courses::list() {
		Ok(()) => println!("\n"),
		Err(why) => println!("Could not list courses: {}\n", why)
	},
	"AVG" => match courses::gpa() {
		Ok(()) => println!("\n"), 
		Err(why) => println!("Could not compute gpa: {}\n", why)
	}, 
	"ADD" => match summative::add() {
		Ok(()) => println!("\n"), 
		Err(why) => println!("Could not add summative: {}\n", why)
	},
	"EDIT" => match summative::edit() {
		Ok(()) => println!("\n"), 
		Err(why) => println!("Could not edit summative: {}\n", why)
	},
	"DEL" => match summative::delete() {
		Ok(()) => println!("\n"),
		Err(why) => println!("Could not delete summative: {}\n", why)
	},
	"TRY" => match summative::try_grade() {
		Ok(()) => println!("\n"), 
		Err(why) => println!("Could not show hypothetical grade {}\n", why)
	},
	"HIDE" => println!("See score without weight of summative"),
	"HELP" => bash::help(), 
	_ => println!("Unrecognized Command\n"), 
	}
}
