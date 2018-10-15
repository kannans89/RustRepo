use std::io;
fn main() {


 let mut choice = String::new();
 println!("Enter you favourite programming language :
 \n 1.Java \n 2.C++ \n 3.Rust\n :");
 io::stdin().read_line(&mut choice).expect("Failed");
 
 //remove /n after enter is pressed
 choice = choice.trim().to_string();
 if choice=="1"{
     println!("You are java programmer");
 }
 else if choice=="2"{
     println!("you are C++ programmer");
 }
 else {
     println!("you are Rust programmer");
 }
    
}