//GradeTracker
//Author: Brian Jiang
//Date Created: 12/6/2019

//Commands: 
//addcourse [coursename]
//rmcourse [coursename] (if it exists)
//view [coursename]
//gpa
//add [summative name][coursename][score (percentage or raw)][weight] 
//try [summative name][coursename][score(percentage or raw)][weight]
//hide [summative name][coursename]

use std::io;
use std::io::prelude::*;
use std::error::Error; 

mod courses;   

fn main() { 
	let mut command = String::new(); 
	println!("Welcome to GradeTracker!"); 
	loop {
		command.clear(); 
		io::stdin().read_line(&mut command)
			.expect("Failed to read line"); 
		process(command.to_string());
	}
}

fn process(command: String) { 
	match command.trim().as_ref() {
	"addcourse" => courses::addcourse(),
	"rmcourse" => println!("Removing course"),
	"view" => println!("View Course Breakdown"), 
	"gpa" => println!("View GPA"), 
	"add" => println!("Add summative"), 
	"try" => println!("View impact of hypothetical score"), 
	"hide" => println!("See score without weight of summative"),
	_ => println!("Unrecognized Command"), 
	}
}