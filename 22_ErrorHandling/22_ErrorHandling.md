# Error Handling

In Rust, errors are grouped into two major categories-

|Sr No |  name    |Description|Usage
|:----:|:----------|:-------|:-------
| 1    | Recoverable     | Errors which can be  handled |Result enum
| 2    | UnRecoverable     | Errors which cannot be handled |panic macro

A recoverable error is an error that can be corrected. A program can  retry the failed operation or specify an alternate course of action when it encounters a recoverable error.Recoverable errors do not cause a program to fail abruptly.An example of a recoverable error is `File Not Found` error.

Unrecoverable errors, cause a program to fail abruptly.A program cannot revert to its normal state if an unrecoverable error occurs.It cannot retry the failed operation or undo the error.An example of an unrecoverable error is trying to access a location beyond the end of an array.

Unlike other programming languages,Rust doesn't have exceptions .It returns an enum` Result<T, E> ` for recoverable errors, while it calls the `panic` macro if the program encounters an unrecoverable error. The `panic` macro causes the program to exit abruptly.

## Panic Macro and Unrecoverable Errors

`panic!` macro allows a program to terminate immediately and provide feedback to the caller of the program. It should be used when a program reaches an unrecoverable state.

```rust
fn main() {
   panic!("Hello");
   println!("End of main"); //unreachable statement
}

```

In the above example,the program will terminate immediately when it encounters the `panic!` macro.

Output:  `thread 'main' panicked at 'Hello', main.rs:3`

### Illustration

```rust
fn main() {
  
   let a = [10,20,30];
   a[10]; //invokes a panic since index 10 cannot be reached
}

```

Output is shown as below

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

A program can invoke the `panic!` macro if business rules are violated as shown in the below example: 

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

The above example returns an error if the value assigned to the varaible is odd.

Output:

```rust
thread 'main' panicked at 'NOT_AN_EVEN', main.rs:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

## Result Enum and Recoverable errors

Enum Result<T,E> can be used to handle recoverable errors.It has two variants **OK** and  **Err**.**T** and **E** are generic type parameters. **T** represents the type of the value that will be returned in a success case within the Ok variant, and **E** represents the type of the error that will be returned in a failure case within the Err variant.

```rust
 enum Result<T,E> {
     OK(T),
     Err(E)
 }


```

Let us understand this with the help of an example: 

```rust
use std::fs::File;
fn main() {

 let f  = File::open("main.jpg"); //this file does not exist
 println!("{:?}",f);

}
```

The program returns  `OK(File)` if file already exists and `Err(Error)` if the file is not found.

`Err(Error { repr: Os { code: 2, message: "No such file or directory" } })`

Let us now see how to handle the Err variant-

The following example handles an error returned while opening file using `match` statement-

 ```rust
  use std::fs::File;

fn main() {

 let f  = File::open("main.jpg"); // main.jpg doesn't exist

 match f {

     Ok(f)=>{
              println!("file found {:?}",f);
     },
     Err(e)=>{
            println!("file not found \n{:?}",e); //handled error
     }
 }

println!("end of main");

}


 ```

Note the program prints *end of the main* event though file was not found. This means the program has handled error gracefully.

Output :

```rust
file not found 
Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }
end of main

```

### Illustration

 The `is_even` function returns an error if the number is not an even number, which is handled by the main() function.

```rust

fn main(){

    let result = is_even(13);

    match result {

        Ok(d)=>{
            println!("no is even {}",d);
        },
        Err(msg)=>{
            println!("Error msg is {}",msg);
        }

    }
      println!("end of main");
}

fn is_even(no:i32)->Result<bool,String>{

   if no%2==0 {
       return Ok(true);
   }
   else  {
    return Err("NOT_AN_EVEN".to_string());
   }

}



```

Note since the main handles error gracefully, the *end of main* statement is printed.

Output:

```rust

Error msg is NOT_AN_EVEN
end of main
```

## unwrap() and expect()

The standard library contains a couple of helper methods that both enums `Result<T,E>` and `Option<T>`  implement. You can use them to simplify error cases where you really do not expect things to fail.In case of success from a method, the "unwrap" function is used to extract the actual result.

|Sr No |  method    |Signature |Description
|:----:|:----------|:-------|:----------|
| 1    | unwrap     | unwrap(self): T     | expects self to be Ok/Some and returns the value contained within. If it's Err or None instead, it raises a panic with the contents of the error displayed.
| 2    | expect     | expect(self, msg: &str): T   | behaves like unwrap, except that it outputs a custom message before panicking in addition to the contents of the error.

### unwrap()

The unwrap() function returns the actual result an operation succeeds. It returns a panic with a default error message if an operation fails. This function is a shorthand for match statement.This is shown in the example below:

```rust

fn main(){

    let result = is_even(10).unwrap();
    println!("result is {}",result);
    println!("end of main");

}

fn is_even(no:i32)->Result<bool,String>{

   if no%2==0 {
       return Ok(true);
   }
   else  {
    return Err("NOT_AN_EVEN".to_string());
   }

}

```

```rust
result is true
end of main

```

Modify the above code to pass an odd number to the is_even() function. The `unwrap()` function will panic and return a default error message as shown below:

```rust
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "NOT_AN_EVEN"', libcore\result.rs:945:5
note: Run with `RUST_BACKTRACE=1` for a backtrace

```

### expect()

The program can return a custom error message in case of a panic. This is shown in the following example:

```rust
use std::fs::File;
fn main(){
    let f = File::open("pqr.txt").expect("File not able to open");//file does not exist
    println!("end of main");
}

```

The function expect()  is similar to  unwrap(). The  only difference is that a custom error message can be displayed using expect.

Output:

```rust

thread 'main' panicked at 'File not able to open: Error { repr: Os { code: 2, message: "No such file or directory" } }', src/libcore/result.rs:860
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

<!--
## Fallible Function

A fallible function is a function which return error if it fails or result if it is success-->
