# Modules

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

