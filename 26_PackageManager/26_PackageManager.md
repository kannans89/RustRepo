# PackageManager

Cargo is Rust's build system and package manager.This is a tool to manage Rust projects.The following are common cargo commands.

 |Sr No |  command    | description|
|:----:|:----------|:-------|
| 1    | cargo build     | compile the current project     |
| 2    | cargo check     | Analyze the current project and report errors, but don't build object files     |
| 3    | cargo run     | Build and execute src/main.rs
|4   | cargo clean |     Remove the target directory
|5|cargo update|  Update dependencies listed in Cargo.lock
|6|cargo new |  Create a new cargo project

Cargo helps to download third party libraries also . So it acts like a package manager.You can also build your own libraries. Cargo is installed by default when you install Rust.

To create a new cargo project we can use the following commands: 
 
 ### Create  a binary crate
 
 `cargo new project_name --bin `
 
 ### Create a library crate 
`cargo new project_name  --lib`

To check the current version of cargo
`cargo --version`

## Illustration: Create a Binary Cargo project

The game  generates an random number and prompts the user to guess the number.

### Step 1: Create a project folder.

Open the terminal and type the following command
`cargo new guess-game-app --bin` .

This will create following folder structure.

```rust
  guess-game-app/
         -->Cargo.toml
         -->src/
           main.rs
```

The  `cargo new` command is used to create a crate. The `--bin` flag indicates that the crate being created is a binary crate.
Public crates are stored in a central repository called crates.io `(https://crates.io/)`.

### Step 2: Include references to external libraries

In this porject we have to use random number.Since the internal standard library does not provide random no gereneration logic we need to look at external libraries or crates. Lets use `rand` crate which is available at crates.io website [crates.io](https://crates.io/)

 The [rand crate](https://crates.io/crates/rand) is a rust library for random number generation.

Rand provides utilities to generate random numbers, to convert them to useful types and distributions, and some randomness-related algorithms

Following diagram shows crate.io website and search result for rand crate.

![crates_io](https://user-images.githubusercontent.com/9062443/47617238-2f44ae00-daeb-11e8-876b-70a4f1248bb6.png)

 Copy the version of rand crate to the Cargo.toml file `rand = "0.5.5"`

```rust
[package]
name = "guess-game-app"
version = "0.1.0"
authors = ["Mohtashim"]

[dependencies]
rand = "0.5.5"

```

### Step 3: Compile the Project 

Navigate to the project folder. Execute the command **cargo build** on the terminal window : 

```rust
 Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading rand v0.5.5
 Downloading rand_core v0.2.2
 Downloading winapi v0.3.6
 Downloading rand_core v0.3.0
   Compiling winapi v0.3.6
   Compiling rand_core v0.3.0
   Compiling rand_core v0.2.2
   Compiling rand v0.5.5
   Compiling guess-game-app v0.1.0 (file:///E:/RustWorks/RustRepo/Code_Snippets/cargo-projects/guess-game-app)
    Finished dev [unoptimized + debuginfo] target(s) in 1m 07s

```

The rand crate and  and all transitive dependencies(inner dependencies of rand) wil be automatically downloaded. 

### Step 4: The Business Logic

 The number guessing game business logic is 
  - Game initially generates an random number
  - Ask user to enter input and guess that number
  - If number is less than acutal it says too low
  - if number is greater than actual it say too high
  - If same number game exits


### Step 5: Edit the main.rs file

Add the business logic to main.rs file.

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

### Step 6: Compile and Execute the Project

Execute the command `cargo run` on the terminal. Make sure that the terminal points to the Project directory.

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
