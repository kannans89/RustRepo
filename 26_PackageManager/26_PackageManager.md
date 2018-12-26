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

To create a new cargo project we can use command `cargo new project_name --bin or --lib` .To create a binary crate project use option --bin and to create a library crate use option --lib.

Cargo helps to download third party libraries also . So it acts like a package manager.You can also build your own libraries. Cargo is installed by default when you install rust.
`cargo --version` command will give the current version of cargo.

## Create a binary Cargo project

**step 1**: open terminal and type the following command
`cargo new guess-game-app --bin` this will create following folder structure.

```rust
  guess-game-app/
         -->Cargo.toml
         -->src/
           main.rs
```

 When you do a `cargo new` you are creating a crate.`--bin` flag creates binary crate .

There is a central repository of public crates called crates.io `(https://crates.io/)`.
In this porject we have to use random number.Since the internal standard library doesnot provide random no gereneration logic we need to look at external libraries or crates. Lets use `rand` crate which is available at crates.io website [crates.io](https://crates.io/)

 The [rand crate](https://crates.io/crates/rand) is a rust library for random number generation.

Rand provides utilities to generate random numbers, to convert them to useful types and distributions, and some randomness-related algorithms

Following diagram shows crate.io website and search result for rand crate.

![crates_io](https://user-images.githubusercontent.com/9062443/47617238-2f44ae00-daeb-11e8-876b-70a4f1248bb6.png)

Step2:
copy the version of rand crate to the Cargo.toml file `rand = "0.5.5"`

```rust
[package]
name = "guess-game-app"
version = "0.1.0"
authors = ["Mohtashim"]

[dependencies]
rand = "0.5.5"

```

step 3:after this fire **cargo build**

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

rand and all transitive dependencies(inner dependencies of rand) also downloaed. it automatically downloaed internal dependencies.

step 4:
 The number guessing game business logic is 
  - Game initially generates an random number
  - Ask user to enter input and guess that number
  - If number is less than acutal it says too low
  - if number is greater than actual it say too high
  - If same number game exits


step 5: Edit the main.rs

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

step 5: Do `cargo run` from project folder to compile and execute.



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

<!-- 
## //To Merge

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

- Rust divides code into crates
- There is a central repository of public crates called crates.io `(https://crates.io/)`
- In rust ecosystem version numbers have meanings
- eg: 1.2.32 (Major.Minor.Patch)
- Major are for breaking changes
- Minor are for new APIs
- Patch versions are for  fixes
  
 //Image from crates.io

## Install dependencies

  For a number guessing game we need to use random number functions , which are not available in the standard library`.https://crates.io/crates/rand`

  how to use rand in  Cargo.toml

  ```rust
    [dependencies]
    rand = "0.6"

  ```

After this do a `cargo build` to download all the dependencies including `rand`
-->
