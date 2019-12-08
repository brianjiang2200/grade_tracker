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
	 
	//Add Course to Course List
	let mut course_list = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.append(true)
		.open("data/courses.txt")
		.expect("Unable to open file"); 
		
	course_list.write_all(course_name.as_bytes());

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