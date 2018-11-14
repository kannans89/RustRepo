extern crate rand;
use rand::Rng;
use std::io;



fn main() {
    println!("Guess a number ?");
    let secret = rand::thread_rng().gen_range(1,10);

    loop {
        println!("Input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Fail");
        let guess:i32 = guess.trim().parse().expect("Fail");

        if guess == secret {
            println!("Guessed Correctly");
            break;
        }else {
            println!("Try again");
            if guess> secret {
                println!("you have guessed in higer no");
            }
            else {
                println!("value is small");
            }
        }
        
    }
}
