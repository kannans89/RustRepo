fn main() {
    
    let mut name = String::from("kannan");
    display(&mut name);
    println!("{}",name);
}

fn display(n:&mut String){
  println!("inside display");
   println!("{}",n);
    println!("{}",*n);
   //*n=String::from("sudhakaran");
   n.push_str(" channapetta");
}