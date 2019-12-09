pub fn bash() {
  println!("addcourse [coursename]"); 
  println!("\tAdd a new course"); 
  
  println!("rmcourse [coursename]"); 
  println!("\tRemove an existing course"); 
  
  println!("view [coursename]"); 
  println!("View current course breakdown"); 
  
  println!("gpa"); 
  println!("\tView cumulative average"); 
  
  println!("add [summativename] [coursename] [score(% or fractional)] [weight(%)]"); 
  println!("\tAdd a new summative"); 
  
  println!("try [summativename] [coursename] [score(% or fractional)] [weight(%)]");
  println!("\tView how a summative would hypothetically impact your mark"); 
  
  println!("hide [summativename][coursename]"); 
  println!("\tView how removing a summative would hypothetically impact your mark"); 
}