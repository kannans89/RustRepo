# Error Handling

In Rust errors are grouped into two major category.

1. Recoverable
2. UnRecoverable

## What is panic macro

```rust
fn main() {
  
   panic!("Hello");
}

```

output `thread 'main' panicked at 'Hello', main.rs:3`

## Unrecoverable Errors with Panic

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

## Recoverable errors with Result

Enum result can be used to handle recoverable errors

syntax shows a generic type Result shows.**T** represents the *Type* of the success result  and **E** represents the *Error* type result.

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

 let f  = File::open("main.rs"); // change extension to main.rust
 println!("{:?}",f);

}
```

output is `Ok(File { fd: 3, path: "/home/cg/root/6728626/main.rs", read: true, write: false })`

output after changing the extension to *main.rust* the extension

`Err(Error { repr: Os { code: 2, message: "No such file or directory" } })`


### Unwrap and Expect