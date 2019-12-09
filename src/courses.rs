use std::io;
use std::io::prelude::*;
use std::error::Error; 
use std::fs;
use std::fs::OpenOptions;
use std::fs::File;
use std::path::Path;

use json::object;

pub fn addcourse() {
	//Enter Course Name
	let mut course_name = String::new(); 
	println!("Course Name/Code:");
	io::stdin().read_line(&mut course_name)
		.expect("Failed to read Course Name");

	//Generate new JSON File 
	course_name = course_name.trim().to_string(); 
	let mut json_file_name = String::from("data/"); 
	json_file_name.push_str(&course_name); 
	json_file_name.push_str(".json"); 
	
	let path = Path::new(&json_file_name); 
	let display = path.display(); 
	
	let mut course_file = match File::create(&path) {
		Err(why) => panic!("couldn't create {}: {}", display, why.description()),
		Ok(course_file) => course_file,
	};
	
	//create JSON object
	let course_object = object!{
		"courseName" => course_name 
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
	let mut json_file_name = String::from("data/"); 
	json_file_name.push_str(&course_name); 
	json_file_name.push_str(".json"); 
	
	fs::remove_file(json_file_name)?; 
	
	Ok(());
}

pub fn view() -> std::io::Result<()> {
	//Enter Course Name 
	let mut course_name = String::new(); 
	println!("Course to View?:"); 
	io::stdin().read_line(&mut course_name)
		.expect("Failed to read course name"); 
	
	//Check if course exists
	course_name = course_name.trim().to_string(); 
	let mut json_file_name = String::from("data/"); 
	json_file_name.push_str(&course_name); 
	json_file_name.push_str(".json");
	
	let contents = fs::read_to_string(json_file_name)?;
	contents = json::parse(contents).unwrap(); 
	
	//print course name
	assert!(contents["courseName"] == course_name);
	println!("{}", course_name\n); 
	
	//print Summatives
	println!("Summatives:");
	
	let mut k = 0; 
	while !contents["Summatives"][k].is_null() {
		println!("\tName: {}", contents["Summatives"][k]["Name"]); 
		println!("\tScore: {}", contents["Summatives"][k]["Score"]); 
		println!("\tWeight: {}", contents["Summatives"][k]["Weight"]);
		println!("\n"); 
		k+=1; 
	}
	
	println!("Current/Projected Average: {}", contents["Average"]); 
	println!("Lazy Average: {}", contents["Lazy Average"]); 
	Ok(()); 
}
