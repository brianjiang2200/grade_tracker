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
	println!("Course Name/Code:"); 
	io::stdin().read_line(&mut course_name)
		.expect("Failed to read Course Name"); 
	course_name = String::from(course_name.trim().to_string()); 
	
	let mut sum_name = String::new();
	let mut score = 0;
	let mut weight = 0;
	
	//Find correct File 
	let json_file_name = jsondata::new_json(&course_name);
	let path = Path::new(&json_file_name);
	
	if path.exists() {
		list(&json_file_name)?; 
		
		//ask for new summative info
		get_summative_info(&mut sum_name, &mut score, &mut weight); 
		
		let contents = fs::read_to_string(json_file_name)?; 
		let deserialized: Course = serde_json::from_str(contents).unwrap(); 
		
		//check if a summative already uses the provided name
		let mut name_taken = false; 
		for entry in &deserialized.Summatives {
			if entry.Name == sum_name {
				name_taken = true; 
				break; 
			}
		}
		
		if !name_taken {
			let new_summative: Summative = Summative {Name: sum_name, Score: score, Weight: weight}; 
			deserialized.Summatives.push(new_summative); 
			let serialized = serde_json::to_string(&deserialized).unwrap(); 
			
			let mut course_file = match File::open(&path) {
				Err(why) => panic!("Could not open file to add summative information...{}", why.description()), 
				Ok(course_file) => course_file,
			}; 
			
			match course_file.write_all(serialized.as_bytes()) {
				Err(why) => panic!("Could not write summative information to file...{}", why.description()), 
				Ok(_) => println!("Successfully added Summative."), 
			}
		}
		else {
			println!("Terminating process...A summative with the same name already exists.");
		}
	}
	else {
		println!("Terminating process...The course {} does not exist.", course_name); 
	}
	
	Ok(())
}

pub fn edit() -> std::io::Result<()> {
	
	let mut course_name = String::new();
	println!("Course Name/Code:"); 
	io::stdin().read_line(&mut course_name)
		.expect("Failed to read Course Name"); 
	course_name = String::from(course_name.trim().to_string()); 
	
	let mut sum_name = String::new();
	let mut score = 0; 
	let mut weight = 0; 

	//Find correct file
	let json_file_name = jsondata::new_json(&course_name);
	let path = Path::new(&json_file_name); 

	if path.exists() {
	
		list(&json_file_name)?; 
		
		let mut index = -1; 
		loop {
			let mut cand = String::new(); 
			println!("Enter the index of the summative to edit from the list above");
			io::stdin().read_line(&mut cand)
				.expect("Failed to read input"); 
			index = match cand.trim().parse() {
				Ok(num) => num, 
				Err(_) => continue,
			};
			break; 
		}
		
		if (index > 0) {
			get_summative_info(&mut sum_name, &mut score, &mut weight); 
			
			let contents = fs::read_to_string(json_file_name)?;
			let course_object: Value = serde_json::from_str(&contents).unwrap();
			
			if !course["Summatives"][index - 1].is_null() {
			
				course_object["Summatives"][index - 1]["Name"] = sum_name.to_ascii_uppercase(); 
				course_object["Summatives"][index - 1]["Score"] = score; 
				course_object["Summatives"][index - 1]["Weight"] = weight;
				
				let mut course_file = match File::open(&path) {
					Err(why) => panic!("Could not open file to add summative information...{}", why.description()),
					Ok(course_file) => course_file,
				}; 
			
				match course_file.write_all(course_object.dump().as_bytes()) {
					Err(why) => panic!("Could not write summative information to file...{}", why.description()),
					Ok(_) => println!("Successfully edited Summative."),
				}
			}
			
			else {
				println!("Terminating Process Gracefully...Invalid Index");
				return Ok(()); 
			}
			
		}
		else {
			return Ok(());
		}
	}
	else {
		println!("Terminating process...The course {} does not exist.", course_name); 
	}
	
	Ok(())
}

pub fn delete() -> std::io::Result<()> {

	let mut course_name = String::new();
	println!("Course Name/Code:"); 
	io::stdin().read_line(&mut course_name)
		.expect("Failed to read Course Name"); 
	course_name = String::from(course_name.trim().to_string()); 
	
	let mut sum_name = String::new();
	let mut score = 0;
	let mut weight = 0;
	
	let mut summative_exists = false; 
	
	//Find correct File 
	let json_file_name = jsondata::new_json(&course_name);
	let path = Path::new(&json_file_name);
	
	if path.exists() {
		list(&json_file_name)?; 
		
		//get summative name only
		println!("Summative Name: ");
		io::stdin().read_line(&mut sum_name)
			.expect("Failed to read summative name.");
		sum_name = String::from(sum_name.trim().to_string()); 
		
		let contents = fs::read_to_string(json_file_name)?; 
		let deserialized: Course = serde_json::from_str(contents).unwrap(); 
		
		for entry in deserialized.Summatives {
			if entry.Name == sum_name {
				summative_exists = true; 
				deserialized.Summatives.remove(entry); 
				let serialized = serde_json::to_string(&deserialized).unwrap(); 
					
				let mut course_file = match File::open(&path) {
					Err(why) => panic!("Could not open file to delete summative info...{}", why.description()),
					Ok(course_file) => course_file,
				};
					
				match course_file.write_all(serialized.as_bytes()) {
					Err(why) => panic!("Could not write update course data to file...{}", why.description()), 
					Ok(_) => println!("Successfully edited course data."),
				}
					
				break;
			}
		}
		
		if !summative_exists {
			println!("Nothing to remove...The summative {} does not exist", sum_name);
		}
	}
	else {
		println!("Terminating process...The course {} does not exist.", course_name);
	}
	
	Ok(())
}

//non public function
fn get_summative_info(sum_name: &mut String, score: &mut u32, weight: &mut u32) { 

	println!("Summative Name: "); 
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

fn list(file_name: &String) -> std::io::Result<()> {
	//should be called only when file is guaranteed to exist
	let contents = fs::read_to_string(file_name)?;
	let course: Value = serde_json::from_str(&contents).unwrap(); 
		
	println!("\nSummatives:"); 
		
	let mut k = 0; 
	while !course[Summatives"][k].is_null() {
		println!("\t{}.Name: {}", k, course[Summatives"][k]["Name"]);
		k += 1;
	}
	
	println!("\n"); 
	Ok(())
}
