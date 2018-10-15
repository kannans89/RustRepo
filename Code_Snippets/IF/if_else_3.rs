use std::io;
fn main(){
    let mut name = String::new();
    let mut age = String::new();
    let mut choice = String::new();

    println!("Enter name and age");

    io::stdin().read_line(&mut name).expect("Failed");
    io::stdin().read_line(&mut age).expect("Failed");

    let age:i32 = age.trim().parse().expect("Failed");
    println!("Do you want to create account(y/n)");
    io::stdin().read_line(&mut choice).expect("Failed");
    choice=choice.trim().to_string();
    if choice=="y" {
        if age<10 {
            println!("Your age is less")
        }
        else{
            println!("Your account is created")
        }
    }

      
}