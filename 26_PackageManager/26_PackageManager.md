# PackageManager

Cargo is package manager of rust . Let us create a sample number guessing game using cargo,there by we will look at the dependencies of cargo as well.

you can open a terminal and type command 
`cargo version` output will be somthing like `cargo 1.29.0 (524a578d7 2018-08-05)`

## create a new project

Fire the command on termainal `cargo new guess-game` , this will create a folder guess-game with following folder structure

```rust
      guess-game
       - src
          main.rs
       - Cargo.toml

```

This will be contents of the Cargo.toml file

```rust
  [package]
name = "guess-game"
version = "0.1.0"
authors = ["mohtashim <mohtashim@tutorialspoint.com>"]

[dependencies]


```

Also the main.rs will contain following contents

```rust
fn main() {
    println!("Hello, world!");
}

```

To run the program we could use `cargo run` after changing directory in terminal to folder `cd guess-game`

output is as shown first program will be compiled and then executed as below.

```rust
  Compiling guess-game v0.1.0 (file:///E:/RustWorks/guess-game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.15s
     Running `target\debug\guess-game.exe`
Hello, world!

```

## Crate Ecosystem

- Rust devides code into crates
- There is a centeral repository of public crates called crates.io (https://crates.io/)
- In rust ecosystem version numbers have meanings
- eg: 1.2.32 (Major.Minor.Patch)
- Major are for breaking changes
- Minor are for new APIs
- Patch versions are for  fixes
  
 //Image from crates.io

  ## Install dependencies

  For a number guessing game we need to use random number functions , which are not available in the standard library.https://crates.io/crates/rand

  how to use rand in  Cargo.toml 

  ```rust
    [dependencies]
    rand = "0.6"

  ```

After this do a `cargo build` to download all the dependencies including `rand`

```rust

use std::io;
extern  crate rand;
use rand::random;

fn get_guess() -> u8 {
    loop{
         println!("Input guess") ;
         let mut guess = String::new();
         io::stdin().read_line(&mut guess)
                   .expect("could not read from stdin");
         match guess.trim().parse::<u8>(){ //remember to trim input to avoid enter spaces
              Ok(v) => return v,
              Err(e) => println!("could not understand input {}",e)
         }
    }
}

fn handle_guess(guess:u8,correct:u8)-> bool {
    if guess < correct {
        println!("Too low");
        false

    }else if guess> correct{
         println!("Too high");
        false

    }
    else {
        println!("You go it ..");
        true
    }

}

fn main() {
    println!("Welcome to no guessing  game");

    let correct:u8 = random();
    println!("correct value is {}",correct);

    
    loop {
        let guess = get_guess();
        if handle_guess(guess,correct){
            break; 
        }
    }

}


```

output is shown below

```rust
Welcome to no guessing  game
correct value is 97
Input guess
20
Too low
Input guess
100
Too high
Input guess
97
You go it ..


````