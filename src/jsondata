use std::error::Error; 
use std::fs; 
use std::fs::OpenOptions; 
use std::fs::File; 
use std::path::Path; 

use json::object; 

pub fn new_json(course_name: String) -> String {
  let mut json_file = String::from("data/"); 
  json_file.push_str(&course_name); 
  json_file.push_str(".json"); 
  
  json_file; 
 }
 
 pub fn extract_name(json_file: String) -> String {
  let contents = fs::read_to_string(json_file).unwrap();
  contents = json::parse(contents).unwrap(); 
  contents["courseName"]; 
 }
