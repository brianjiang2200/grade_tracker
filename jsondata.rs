#![allow(non_snake_case)]
#![allow(dead_code)]

use std::fs;
use serde::{Serialize, Deserialize};
use serde_json::Value;   

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
 
 #[derive(Serialize, Deserialize)]
 pub struct Course {
	pub courseName: String, 
	pub Average: f64, 
	pub Lazy: f64, 
	pub Summatives: Vec<Summative>, 
}

#[derive(Serialize, Deserialize)]
pub struct Summative {
	pub Name: String, 
	pub Score: f64,
	pub Weight: f64,
}