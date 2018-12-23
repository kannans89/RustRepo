# PackageManager

## Introduction to Cargo

Cargo is Rust's build system and package manager.This is a tool to manage Rust projects.Cargo commands
 `cargo build` - to build or compile projects.
 `cargo check`
 `cargo run`
Cargo helps to dowload third party libraries also . So it acts like a package manager.
You can also build your own libraries. Cargo is installed by default when you install rust.
`cargo --version`
create a new project using cargo
`cargo new project_name --bin or --lib`
--bin applications can be executed
--lib cannot be executed , it is re usable like a DLL in windows.

## Create a binary Cargo project

step 1: open terminal and type the following command
`cargo new guess-game-app --bin` this will create following folder structure.

```rust
  guess-game-app/
         -->Cargo.toml
         -->src/
           main.rs
```

## Crates

- Crate is a package of Rust code
- Crate can be binary or library. When you do a `cargo new` you are creating a crate.`--bin` flag creates binary crate and  `--lib` creates library.

Lets use `rand` crate which is available at crates.io website [crates.io](https://crates.io/)

 [rand crate](https://crates.io/crates/rand)
 A Rust library for random number generation.

Rand provides utilities to generate random numbers, to convert them to useful types and distributions, and some randomness-related algorithms

![crates_io](https://user-images.githubusercontent.com/9062443/47617238-2f44ae00-daeb-11e8-876b-70a4f1248bb6.png)

copy this to the Cargo.toml file `rand = "0.5.5"`

```rust
[package]
name = "guess-game-app"
version = "0.1.0"
authors = ["Mohtashim"]

[dependencies]
rand = "0.5.5"

```

after this fire **cargo build**

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

rand and all transivtive dependencis also downloaed. it automatically downloaed internal dependencies.


## //To Merge..

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