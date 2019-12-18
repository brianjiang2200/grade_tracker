/*******************DEFINED IN SUMMATIVE MODULE********************
pub add()
pub edit()
pub delete()
pub try_grade()
pub hide_grade()
get_summative_info()
compute_course_averages()
list()
*******************************************************************/

use std::io;
use std::io::prelude::*;
use std::error::Error; 
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path; 

use serde_json::Value;
use serde::{Serialize, Deserialize}; 

#[path = "jsondata.rs"]
pub mod jsondata;
use crate::jsondata::{Course, Summative};  

pub fn add() -> std::io::Result<()> {

	let mut course_name = String::new();
	println!("Course Name/Code:"); 
	io::stdin().read_line(&mut course_name)
		.expect("Failed to read Course Name"); 
	course_name = String::from(course_name.trim().to_string()); 
	
	let mut sum_name = String::new();
	let mut score = 0.0;
	let mut weight = 0.0;
	
	//Find correct File 
	let json_file_name = jsondata::new_json(&course_name);
	let path = Path::new(&json_file_name);
	
	if path.exists() {
		list(&json_file_name)?;
		
		//ask for new summative info
		get_summative_info(&mut sum_name, &mut score, &mut weight); 
		
		let contents = fs::read_to_string(&json_file_name)?; 
		let mut deserialized: Course = serde_json::from_str(&contents)?; 
		
		//check if a summative already uses the provided name
		let mut name_taken = false; 
		for entry in &deserialized.Summatives {
			if entry.Name == sum_name.to_ascii_uppercase() {
				name_taken = true; 
				break; 
			}
		}
		
		if !name_taken {
			//push new summative
			let new_summative: Summative = Summative {
				Name: sum_name.to_ascii_uppercase(), 
				Score: score, 
				Weight: weight
			}; 
			deserialized.Summatives.push(new_summative);
			
			//compute new averages
			deserialized.Lazy = 0.0; 
			let mut cumulative = 0.0; 
			for member in &deserialized.Summatives {
				cumulative += member.Score; 
				deserialized.Lazy += member.Score * member.Weight / 100.0; 
			}
			deserialized.Average = cumulative / deserialized.Summatives.len() as f64;
			
			let serialized = serde_json::to_string(&deserialized).unwrap();
			
			let mut course_file = OpenOptions::new()
				.read(true)
				.write(true)
				.open(&path)
				.expect("Found course file...but it failed to open.");
				
			course_file.set_len(0)?;
			
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
	let mut score = 0.0; 
	let mut weight = 0.0; 

	//Find correct file
	let json_file_name = jsondata::new_json(&course_name);
	let path = Path::new(&json_file_name); 

	if path.exists() {
	
		list(&json_file_name)?; 
		
		let mut index = 0; 
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
		
		if index > 0 {
			get_summative_info(&mut sum_name, &mut score, &mut weight); 
			
			let contents = fs::read_to_string(&json_file_name)?;
			let mut deserialized: Course = serde_json::from_str(&contents)?;
			
			if index <= deserialized.Summatives.len() {
			
				deserialized.Summatives[index - 1].Name = sum_name.to_ascii_uppercase(); 
				deserialized.Summatives[index - 1].Score = score; 
				deserialized.Summatives[index - 1].Weight = weight;
				
				//compute new averages
				deserialized.Lazy = 0.0; 
				let mut cumulative = 0.0; 
				for member in &deserialized.Summatives {
					cumulative += member.Score; 
					deserialized.Lazy += member.Score * member.Weight / 100.0; 
				}
				deserialized.Average = cumulative / deserialized.Summatives.len() as f64;
				
				let serialized = serde_json::to_string(&deserialized).unwrap(); 
				
				let mut course_file = OpenOptions::new()
					.read(true)
					.write(true)
					.open(&path)
					.expect("Found course file...but it failed to open."); 
				
				course_file.set_len(0)?;
				
				match course_file.write_all(serialized.as_bytes()) {
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
	
	//Find correct File 
	let json_file_name = jsondata::new_json(&course_name);
	let path = Path::new(&json_file_name);
	
	if path.exists() {
	
		list(&json_file_name)?; 
		
		let mut index = 0; 
		loop {
			let mut cand = String::new(); 
			println!("Enter the index of the summative to delete from the list above");
			io::stdin().read_line(&mut cand)
				.expect("Failed to read input"); 
			index = match cand.trim().parse() {
				Ok(num) => num, 
				Err(_) => continue,
			};
			break; 
		}
		
		if index > 0 { 
			
			let contents = fs::read_to_string(&json_file_name)?;
			let mut deserialized: Course = serde_json::from_str(&contents)?;
			
			if index <= deserialized.Summatives.len() {
			
				deserialized.Summatives.remove(index - 1);
				
				//compute new averages
				deserialized.Lazy = 0.0; 
				let mut cumulative = 0.0; 
				for member in &deserialized.Summatives {
					cumulative += member.Score; 
					deserialized.Lazy += member.Score * member.Weight / 100.0; 
				}
				if deserialized.Summatives.len() > 0 {
					deserialized.Average = cumulative / deserialized.Summatives.len() as f64;
				}
				
				let serialized = serde_json::to_string(&deserialized).unwrap(); 
				
				let mut course_file = OpenOptions::new()
					.read(true)
					.write(true)
					.open(&path)
					.expect("Found course file...but it failed to open.");
					
				course_file.set_len(0)?; 
			
				match course_file.write_all(serialized.as_bytes()) {
					Err(why) => panic!("Could not write summative information to file...{}", why.description()),
					Ok(_) => println!("Successfully deleted Summative."),
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

pub fn try_grade() -> std::io::Result<()> {
	let mut course_name = String::new();
	println!("Course Name/Code:"); 
	io::stdin().read_line(&mut course_name)
		.expect("Failed to read Course Name"); 
	course_name = String::from(course_name.trim().to_string()); 
	
	let mut sum_name = String::new();
	let mut score = 0.0;
	let mut weight = 0.0;
	
	//Find correct File 
	let json_file_name = jsondata::new_json(&course_name);
	let path = Path::new(&json_file_name);
	
	if path.exists() {
		list(&json_file_name)?;
		
		//ask for new summative info
		get_summative_info(&mut sum_name, &mut score, &mut weight); 
		
		let contents = fs::read_to_string(&json_file_name)?; 
		let mut deserialized: Course = serde_json::from_str(&contents)?;
		
		//push new summative
		let new_summative: Summative = Summative {
			Name: sum_name.to_ascii_uppercase(), 
			Score: score, 
			Weight: weight
		}; 
		deserialized.Summatives.push(new_summative);
			
		//compute new averages
		deserialized.Lazy = 0.0; 
		let mut cumulative = 0.0; 
		for member in &deserialized.Summatives {
			cumulative += member.Score; 
			deserialized.Lazy += member.Score * member.Weight / 100.0; 
		}
		deserialized.Average = cumulative / deserialized.Summatives.len() as f64;
		
		//DO NOT SAVE TO FILE
		
		println!("Hypothetical Average: {}", deserialized.Average); 
		println!("Lazy Average: {}", deserialized.Lazy);
		
	}
	else {
		println!("Terminating process...The course {} does not exist.", course_name); 
	}
		
	Ok(())
}

pub fn hide_grade() -> std::io::Result<()> {
	let mut course_name = String::new();
	println!("Course Name/Code:"); 
	io::stdin().read_line(&mut course_name)
		.expect("Failed to read Course Name"); 
	course_name = String::from(course_name.trim().to_string()); 
	
	//Find correct File 
	let json_file_name = jsondata::new_json(&course_name);
	let path = Path::new(&json_file_name);
	
	if path.exists() {
		list(&json_file_name)?;
		
		let mut index = 0; 
		loop {
			let mut cand = String::new(); 
			println!("Enter the index of the summative to hide from the list above");
			io::stdin().read_line(&mut cand)
				.expect("Failed to read input"); 
			index = match cand.trim().parse() {
				Ok(num) => num, 
				Err(_) => continue,
			};
			break; 
		} 
			
		if index > 0 { 
			
			let contents = fs::read_to_string(&json_file_name)?;
			let mut deserialized: Course = serde_json::from_str(&contents)?;
			
			if index <= deserialized.Summatives.len() {
			
				deserialized.Summatives.remove(index - 1);
				
				//compute new averages
				deserialized.Lazy = 0.0; 
				let mut cumulative = 0.0; 
				for member in &deserialized.Summatives {
					cumulative += member.Score; 
					deserialized.Lazy += member.Score * member.Weight / 100.0; 
				}
				if deserialized.Summatives.len() > 0 {
					deserialized.Average = cumulative / deserialized.Summatives.len() as f64;
				}
				
				//DO NOT SAVE TO FILE
				
				println!("Hypothetical Average: {}", deserialized.Average); 
				println!("Lazy Average: {}", deserialized.Lazy);
			}
			else {
				println!("Terminating process gracefully...Invalid index."); 
			}
		}
	}
	else {
		println!("Terminating process...The course {} does not exist.", course_name); 
	}
		
	Ok(())
}

//non public function
fn get_summative_info(sum_name: &mut String, score: &mut f64, weight: &mut f64) { 

	println!("Summative Name: "); 
	io::stdin().read_line(sum_name)
		.expect("Failed to read Summative Name");
	*sum_name = String::from(sum_name.trim().to_string()); 
	
	//loop process while input is invalid
	loop {
		let mut cand = String::new();
		println!("Score (% Or Fractional):"); 
		io::stdin().read_line(&mut cand)
			.expect("Failed to Read Score"); 
		*score = match cand.trim().parse() {
			Ok(num) => num, 
			Err(_) => continue,
		};
		break; 
	}
		
	//loop process while input is invalid
	loop {
		let mut cand = String::new(); 
		println!("Weight (%):"); 
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
		
	println!("Existing Summatives:"); 
		
	println!("\t0. Cancel Current Operation"); 
	let mut k = 0; 
	while !course["Summatives"][k].is_null() {
		println!("\t{}. {}", k + 1, course["Summatives"][k]["Name"]);
		k += 1;
	}
	
	println!("\n"); 
	Ok(())
}
