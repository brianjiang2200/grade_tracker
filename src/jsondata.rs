use std::fs;
use serde_json::Value;   

#[allow(dead_code)]

pub fn new_json(course_name: &String) -> String {
  let mut json_file = String::from("data/"); 
  json_file.push_str(&course_name); 
  json_file.push_str(".json"); 
  
  json_file.to_ascii_uppercase()
 }
 
 pub fn extract_name(json_file: &String) -> String {
  let contents = fs::read_to_string(json_file).unwrap();
  let course: Value = serde_json::from_str(&contents).unwrap();
  
  course["courseName"].to_string() 
 }