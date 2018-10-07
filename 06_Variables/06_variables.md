# Variables

A variable provides us with named storage that our programs can manipulate. Each variable in Rust has a specific type, which determines the size and layout of the variable's memory; the range of values that can be stored within that memory;and the set of operations that can be applied to the variable.

The name of a variable can be composed of letters, digits, and the underscore character. It must begin with either a letter or an underscore. Upper and lowercase letters are distinct because Rust is case-sensitive .

## Syntax

When you define a variable the type is optional in Rust.It will take the type from the value assigned.The syntax is given below.

```rust
  let variable_name=value;// no type specified
  let variable_name:dataType = value; //type specified
```

A simple example would be as given below.

```rust
 fn main() {
  let fees = 25_000;
  let salary:f64=35_000.00;
 println!("fees is {} and salary is {}",fees,salary);
}

```

output will be `fees is 25000 and salary is 35000`.

## Immutable

By default variables are immutable or read only in Rust by design.When a variable is immutable, once a value is bound to a variable name, you can’t change that value.
Let us understand this with an example.

```rust
  fn main() {
  let fees = 25_000;
  println!("fees is {} ",fees);
  fees=35_000;
  println!("fees changed is {}",fees);
}

```

output will be like this

```rust
 error[E0384]: re-assignment of immutable variable `fees`
 --> main.rs:6:3
  |
3 |   let fees = 25_000;
  |       ---- first assignment to `fees`
...
6 |   fees=35_000;
  |   ^^^^^^^^^^^ re-assignment of immutable variable

error: aborting due to previous error(s)

```

The error message indicates that the cause of the error is that you cannot assign twice to immutable variable *fees*, because you tried to assign a second value to the immutable *fees* variable.This is one of many ways Rust allows programmers to  write  code in a way that takes advantage of the safety and easy concurrency .

## Mutable

Mutable variable values can be changed . Variables are immutable by default but we can apply a keyword **mut** in front of variable to make it mutable.Syntax is as shown below.

```rust
 let mut variable_name=value;
 let mut variable_name:dataType=value;
```

Let us understand this with an example

```rust
 fn main() {
  let mut fees:i32 = 25_000;
  println!("fees is {} ",fees);
  fees=35_000;
  println!("fees changed is {}",fees);
}

```

output will be as shown

```rust
  fees is 25000
  fees changed is 35000
```