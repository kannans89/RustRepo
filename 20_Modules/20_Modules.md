# Modules

Modules helps to organize  the code into logical groups.For example *network* module contains networking related functions and *graphics* module contains drawing related functions.Modules are similar to  namespaces in other programming languages.

The package system in Rust is called **Cargo** and packages themselves are called **Crates** .Rust programs are made of crates. Each crate is a Rust project: all the source code for a single library or executable, plus any associated tests, examples, tools, configuration.Library crates are project which are to be reused in another project,they doesnt have main to execute.

Let us see the syntax of module.

```rust
     //public module
   pub mod a_public_module{
      pub fn a_public_function(){
          //public function
      }

      fn a_private_function(){
             //private function
      }
   }

   //private module
   mod a_private_module{
         fn a_private_function(){

         }
   }

```

Modules should be prefixed with `pub` keyword to make it public so that it can be accessible outside the module.Let us see an example.

```rust
  
pub mod movies {
            pub fn play(name:String){
                println!("Playing  movie {}",name);
            }
}

fn main(){
    movies::play("Herold and Kumar".to_string());
}

```

## Use Keyword

Modules can also be nested as shown below example .If we want to call play method it will be difficult to remember the full module path like `movies::english::comedy::play`.There is an easy way to solve this problem with `use` keyword.

```rust
pub mod movies {
    pub mod english {
        pub mod comedy{
            pub fn play(name:String){
                println!("Playing comedy movie {}",name);
            }
        }
    }
}

use movies::english::comedy::play;

fn main(){
   // short path syntax
   play("Herold and Kumar".to_string());
   play("The Hangover".to_string());

   //full path syntax
   movies::english::comedy::play("Airplane!".to_string());
}

```

## Create a Library Crate and Consume in another Crate

Let us create a library named **movie_lib** which contains a module **movies**.To build the crate library created we will use the tool **cargo**.
First create a folder *movie-lib* , the source code should go in an *src* folder.The cargo tool will look for a file named *Cargo.toml*,this file will contain the metadata of project ,like version number,author name etc.

Project structure is shown below ,src folder has lib.rs and moives.rs

```rust
  movie-lib/
         -->Cargo.toml
         -->src/
           lib.rs
           movies.rs
```

- Now let us first add metadata to project so edit Cargo.toml as below

```rust
[package]
name="movies_lib"
version="0.1.0"
authors = ["Mohtashim"]

```

- now we will edit the **lib.rs** file , which will contain the module definition

```rust
  pub mod movies;
```
This means current project has a module moives but its definition is in another file **movies.rs** as given below

```rust

pub fn play(name:String){
                println!("Playing  movie {} :movies-app",name);
            }

```

After completing all this do a **cargo build** to make sure library is structured propertly.Make sure you should be at root of project , that is movie-app folder  to fire the command . You will get compiling and finished info on the terminal as shown.

```rust
D:\Rust\movie-lib> cargo build
   Compiling movies_lib v0.1.0 (file:///D:/Rust/movie-lib)
    Finished dev [unoptimized + debuginfo] target(s) in 0.67s

```

- Now to consume this library we need to create another project.Let us call project as **movie-lib-test** create this in same root of **movie-lib**. This project should have main method as this will be hosting the library. The folder structre is as shown.


```rust
  movie-lib // already completed

  movie-lib-test/
         -->Cargo.toml
         -->src/
           main.rs
```

Since this is binary project src folder will contain main.rs and not lib.rs.

Add contents of **Cargo.toml** as below

```rust
 [package]
name = "test_for_movie_lib"
version = "0.1.0"
authors = ["Mohtashim"]

[dependencies]
movies_lib = { path = "../movie-lib" }

```

Note that in dependencies we are giving path of the library folder.Make sure both projects are in same folder as shown in hierarchy.

Now let us complete the **main.rs** file as below

```rust
  
extern crate movies_lib;

use movies_lib::movies::play;

fn main(){
    println!("inside main of test ");
    play("Tutorialspoint".to_string())
}

```

In this code we are importing an external package called *movies_lib*.Check the Cargo.toml of current project to verify the crate name.

- Now we need to build this binary project and execute it as shown
using cargo build and cargo run.

![output](https://raw.githubusercontent.com/kannans89/RustRepo/master/Images/20_modules.PNG)