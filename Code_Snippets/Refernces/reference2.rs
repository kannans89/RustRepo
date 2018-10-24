fn main() {
    
    let mut name = String::from("kannan");
    display(&mut name);
    println!("{}",name);
}

fn display(n:&mut String){//n points to name
  println!("inside display");
   println!("{}",n);
    println!("{}",*n);
    *n=String::from("sudhakaran");
   
}