pub fn help() {
  println!("addcourse [coursename]"); 
  println!("\tAdd a new course"); 
  
  println!("rmcourse [coursename]"); 
  println!("\tRemove an existing course"); 
  
  println!("view [coursename]"); 
  println!("\tView current course breakdown");
  
  println!("list"); 
  println!("\tList all registered courses"); 
  
  println!("gpa"); 
  println!("\tView cumulative average"); 
  
  println!("add [summativename] [coursename] [score(% or fractional)] [weight(%)]"); 
  println!("\tAdd a new summative"); 
  
  println!("try [summativename] [coursename] [score(% or fractional)] [weight(%)]");
  println!("\tView how a summative would hypothetically impact your mark"); 
  
  println!("hide [summativename][coursename]"); 
  println!("\tView how removing a summative would hypothetically impact your mark"); 
}
