use std::io;
use std::io::prelude::*;
use std::error::Error; 
use std::fs;
use std::fs::File; 
use std::fs::DirEntry;
use std::path::Path;
use std::env;

use json; 
use json::object;
use serde_json::Value; 

#[path = "jsondata.rs"]
mod jsondata;  

pub fn addcourse() {
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
	
	let mut course_file = match File::create(&path) {
		Err(why) => panic!("couldn't create {}: {}", display, why.description()),
		Ok(course_file) => course_file,
	};
	
	//create JSON object
	let course_object = object!{
		"courseName" => course_name,
		"Average" => "N/A",
		"Lazy Average" => "N/A"
	};
	
	match course_file.write_all(course_object.dump().as_bytes()) {
		Err(why) => panic!("couldn't write to {}: {}", display, why.description()), 
		Ok(_) => println!("successfully added course."), 
	}
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
	
	fs::remove_file(json_file_name)?; 
	
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
		println!("\n"); 
		println!("Summatives:");
	
		let mut k = 0; 
		while !course["Summatives"][k].is_null() {
			println!("\tName: {}", course["Summatives"][k]["Name"]); 
			println!("\tScore: {}", course["Summatives"][k]["Score"]); 
			println!("\tWeight: {}", course["Summatives"][k]["Weight"]);
			println!("\n"); 
			k+=1; 
		}
	
		println!("Current/Projected Average: {}", course["Average"]); 
		println!("Lazy Average: {}", course["Lazy Average"]);
	}
	
	else {
		println!("The course specified does not exist."); 
	}
	Ok(()) 
}

pub fn list() -> std::io::Result<()> {
	let path = Path::new("data"); 
	let paths = fs::read_dir(&path)?; 
	let names = 
	paths.filter_map(|entry| {
		entry.ok().and_then(|e|
			e.path().file_name()
				.and_then(|n| n.to_str().map(|s| String::from(s)))
	)
	}).collect::<Vec<String>>();
	for elem in &names {
		let course_name = jsondata::extract_name(&elem); 
		println!("{}", course_name); 
	}
	Ok(())
}