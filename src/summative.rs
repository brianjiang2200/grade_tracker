use std::io;
use std::error::Error; 
use std::fs;
use std::fs::File;
use std::path::Path; 
 
use serde_json::Value;
use serde::{Serialize, Deserialize}; 

#[path = "jsondata.rs"]
mod jsondata; 

pub fn add() -> std::io::Result<()> {

	let mut course_name = String::new(); 
	let mut sum_name = String::new();
	let mut score = 0;
	let mut weight = 0;
	
	get_summative_info(&mut course_name, &mut sum_name, &mut score, &mut weight); 
	
	//Find correct File 
	let json_file_name = jsondata::new_json(&course_name);
	let path = Path::new(&json_file_name); 
	
	Ok(())
}

//non public function
fn get_summative_info(course_name: &mut String, sum_name: &mut String, score: &mut u32, weight: &mut u32) { 
	println!("Course Name/Code:");
	io::stdin().read_line(course_name)
		.expect("Failed to read Course Name");
	*course_name = String::from(course_name.trim().to_string()); 
		
	println!("Summative Name:"); 
	io::stdin().read_line(sum_name)
		.expect("Failed to read Summative Name");
	*sum_name = String::from(sum_name.trim().to_string()); 
	
	//loop process while input is invalid
	loop {
		let mut cand = String::new();
		println!("Score (% or fractional):"); 
		io::stdin().read_line(&mut cand)
			.expect("Failed to read score"); 
		*score = match cand.trim().parse() {
			Ok(num) => num, 
			Err(_) => continue,
		};
		break; 
	}
		
	//loop process while input is invalid
	loop {
		let mut cand = String::new(); 
		println!("Weight (percentage):"); 
		io::stdin().read_line(&mut cand)
			.expect("Failed to read weight"); 
		*weight = match cand.trim().parse() {
			Ok(num) => num, 
			Err(_) => continue,
		}; 
		break; 
	}
}

fn list(file_name: String) -> std::io::Result<()> {
	//should be called only when file is guaranteed to exist
	let contents = fs::read_to_string(file_name)?;
	let course: Value = serde_json::from_str(&contents).unwrap(); 
		
	println!("\nSummatives:"); 
		
	let mut k = 0; 
	while !course[Summatives"][k].is_null() {
		println!("\tName: {}", course[Summatives"][k]["Name"]);
		k += 1;
	}
}
