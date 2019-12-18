/**********************DEFINED IN COURSES MODULE**********************
pub addcourse()
pub rmcourse()
pub view()
pub list()
pub gpa()
*********************************************************************/

use std::io;
use std::io::prelude::*;
use std::error::Error; 
use std::fs;
use std::fs::File;
use std::path::Path; 

use glob::glob; 

use serde::{Serialize, Deserialize}; 
use serde_json::Value; 

#[path = "jsondata.rs"]
mod jsondata;
use crate::jsondata::{Course, Summative}; 

pub fn addcourse() -> std::io::Result <()> {
	//Enter Course Name
	let mut course_name = String::new(); 
	println!("Course Name/Code:");
	io::stdin().read_line(&mut course_name)
		.expect("Failed to read Course Name");

	//Generate new JSON File 
	course_name = course_name.trim().to_string(); 
	let json_file_name = jsondata::new_json(&course_name);  
	
	let path = Path::new(&json_file_name); 
	let display = path.display(); 
	
	if !path.exists() {
		let mut course_file = match File::create(&path) {
			Err(why) => panic!("couldn't create {}: {}", display, why.description()),
			Ok(course_file) => course_file,
		};
		
		let newcourse: Course = Course { 
			courseName: course_name.to_ascii_uppercase(),
			Average: 0.0, 
			Lazy: 0.0, 
			Summatives: Vec::new()
		};
		let serialized = serde_json::to_string(&newcourse).unwrap(); 
		
		match course_file.write_all(serialized.as_bytes()) {
			Err(why) => panic!("Couldn't write to {}: {}.", display, why.description()), 
			Ok(_) => println!("Successfully added course."), 
		}
	}
	else {
		println!("A course by this name already exists."); 
	}
	
	Ok(())
}

pub fn rmcourse() -> std::io::Result<()> {
	//Enter Course Name
	let mut course_name = String::new();
	println!("Course to Remove?"); 
	io::stdin().read_line(&mut course_name)
		.expect("Failed to read course name"); 
	
	//delete JSON File
	course_name = course_name.trim().to_string(); 
	let json_file_name = jsondata::new_json(&course_name);  
	let path = Path::new(&json_file_name); 
	
	if path.exists() {
		fs::remove_file(json_file_name)?;
		println!("Course {} was successfully removed.", course_name); 
	}
	else {
		println!("Nothing to remove...the course specified does not exist."); 
	}
	
	Ok(())
}

pub fn view() -> std::io::Result<()> {
	//Enter Course Name 
	let mut course_name = String::new(); 
	println!("Course to View?:"); 
	io::stdin().read_line(&mut course_name)
		.expect("Failed to read course name");		
	
	course_name = course_name.trim().to_string(); 
	let json_file_name = jsondata::new_json(&course_name);

	//Check if course exists
	if Path::new(&json_file_name).exists() {
	
		let contents = fs::read_to_string(json_file_name)?;
		let course: Value = serde_json::from_str(&contents).unwrap();  

		//print Summatives 
		println!("\nSummatives:");
	
		let mut k = 0; 
		while !course["Summatives"][k].is_null() {
			println!("\tName: {}", course["Summatives"][k]["Name"]); 
			println!("\tScore: {}", course["Summatives"][k]["Score"]); 
			println!("\tWeight: {}", course["Summatives"][k]["Weight"]);
			println!("\n"); 
			k+=1; 
		}
	
		println!("Current/Projected Average: {}", course["Average"]); 
		println!("Lazy Average: {}", course["Lazy"]);
	}
	
	else {
		println!("The course specified does not exist."); 
	}
	Ok(()) 
}

pub fn list() -> std::io::Result<()> {
	//Initialize Vector to contain string copies of file paths
	let mut file_vec = Vec::new(); 
	//Get all files in directory data with extension .JSON
	for entry in glob("data/*.JSON").expect("Failed to read glob pattern") {
		match entry {
			Ok(path) => {
				let file_name = path.to_string_lossy().into_owned(); 
				file_vec.push(file_name); 
			},
			Err(e) => println!("Could not get path: {}", e) 
		}
	}
	for member in &file_vec {
		println!("{}", jsondata::extract_name(&member).to_ascii_uppercase()); 
	}
	Ok(())
}

pub fn gpa() -> std::io::Result<()> {
	let mut cumulative = 0.0; 
	let mut course_count = 0;
	for entry in glob("data/*.JSON").expect("Failed to read glob pattern") {
		match entry {
			Ok(path) => {
				let mystr = path.to_string_lossy().into_owned(); 
				let contents = fs::read_to_string(mystr)?;
				let course: Value = serde_json::from_str(&contents).unwrap(); 
				cumulative += match course["Average"].as_f64() {
					Some(num) => num, 
					None => 0.0
				};
				course_count += 1;
			},
			Err(e) => println!("Could not get path: {}", e) 
		}
	}
	if course_count > 0 {
		let average = cumulative / course_count as f64;
		println!("{}", average);
	}
	else {
		println!("No courses to compute GPA"); 
	}
	
	Ok(())
}
