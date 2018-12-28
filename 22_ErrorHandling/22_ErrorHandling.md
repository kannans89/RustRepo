# Error Handling

In Rust, errors are grouped into two major category.

|Sr No |  name    |Description|Usage
|:----:|:----------|:-------|:-------
| 1    | Recoverable     | Errors which can be  handled |Result enum
| 2    | UnRecoverable     | Errors which cannot be handled |panic macro

For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

Rust doesn't have exceptions unlike other programming languages.Instead it used ` Result<T, E> ` for recoverable errors and the `panic! `macro that stops execution when the program encounters an unrecoverable error.

## Panic Macro and Unrecoverable Errors

`panic!` macro allows a program to to terminate immediately and provide feedback to the caller of the program. It should be used when a program reaches an unrecoverable problem.

```rust
fn main() {
   panic!("Hello");
   println!("End of main"); //unreachable statement
}

```

In the above example `println` will not be called as after panic the program will terminate immediately.

output `thread 'main' panicked at 'Hello', main.rs:3`

### Illustration

Following program shows panic at runtime . The program on compilation will give a warning *index out of bounds* and run time it throws a panic.

```rust
fn main() {
  
   let a = [10,20,30];
   a[10];
}

```

output is shown below

```rust
warning: this expression will panic at run-time
 --> main.rs:4:4
  |
4 |    a[10];
  |    ^^^^^ index out of bounds: the len is 3 but the index is 10

$main
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 10', main.rs:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.


```

The following program shows how to explicitly use panic macro and throw exception

```rust

 fn main() {
  
  let no = 13; //try with odd and even
  
  if no%2 == 0 {
      println!("Thank you , number is even");
  }
  else {
      panic!("NOT_AN_EVEN");
  }
  
  println!("End of main");
}

```

output for odd no

```rust
thread 'main' panicked at 'NOT_AN_EVEN', main.rs:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

output for even no

```rust
Thank you , number is even
End of main
```

## Result Enum and Recoverable errors

Enum Result<T,E> can be used to handle recoverable errors

Syntax shows a generic type Result enumeration.**T** represents the *Type* of the success result  and **E** represents the *Error* type result.

```rust
 enum Result<T,E> {
     OK(T),
     Err(E)
 }


```

To demonstrate let us see an example where we loading a file

```rust
use std::fs::File;
fn main() {

 let f  = File::open("main.rs"); // edit extension to main.jpg
 println!("{:?}",f);

}
```

The program returns  `OK(File)` if file already exists and `Err(Error)` incase of file not found.

output is `Ok(File { fd: 3, path: "/home/cg/root/6728626/main.rs", read: true, write: false })`

output after changing the extension to *main.jpg* the extension

`Err(Error { repr: Os { code: 2, message: "No such file or directory" } })`

### unwrap() and expect()

The standard library contains a couple of helper methods that both enums `Result<T,E>` and `Option<T>`  implement. You can use them to simplify error cases where you really do not expect things to fail.

|Sr No |  method    |Signature |Description
|:----:|:----------|:-------|:----------|
| 1    | unwrap     | unwrap(self): T     | expects self to be Ok/Some and returns the value contained within. If it's Err or None instead, it raises a panic with the contents of the error displayed.
| 2    | expect     | expect(self, msg: &str): T   | behaves like unwrap, except that it outputs a custom message before panicking in addition to the contents of the error.

- Following shows example to use unwrap

```rust
use std::fs::File;
fn main(){
    let f = File::open("pqr.txt").unwrap();
    println!("end of main");
}
```

The program will not reach till `println!("end of main");` since no file is found the program will
panic . So unwrap() is a short hand for panic if file opening fails. Following error will be displayed.

```rust
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os { code: 2, message: "No such file or directory" } }', src/libcore/result.rs:860
note: Run with `RUST_BACKTRACE=1` for a backtrace.

```

- Following shows example to use `expect`

```rust
use std::fs::File;
fn main(){
    let f = File::open("pqr.txt").expect("File not able to open");
    println!("end of main");
}

```

expect() works same as unwrap() only difference is error message can be passed.The error is displayed as shown.

```rust

thread 'main' panicked at 'File not able to open: Error { repr: Os { code: 2, message: "No such file or directory" } }', src/libcore/result.rs:860
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
