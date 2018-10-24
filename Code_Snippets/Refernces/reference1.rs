fn main() {
    
    let name = String::from("kannan");
    display(&name);
    println!("{}",name);
}

fn display(n:&String){
  println!("inside display");
   println!("{}",n);
    println!("{}",*n);
}