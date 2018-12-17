# Variables

A variable is a named storage that  programs can manipulate.Simply put, a variable helps programs to store values.
Variables in Rust are associated with a specific data type. The data type determines the size and layout of the variable's memory, the range of values that can be stored within that memory and the set of operations that can be performed on the variable.

## Variable Naming-Rules

- The name of a variable can be composed of letters, digits, and the underscore character.
- It must begin with either a letter or an underscore.
- Upper and lowercase letters are distinct because Rust is case-sensitive .

## Syntax

The data type is optional while declaring a variable in Rust. The data type is inferred from the value assigned to the variable.

The syntax for declaring a variable is given below.

```rust
  let variable_name=value;// no type specified
  let variable_name:dataType = value; //type specified
```

### Illustration

```rust
 fn main() {
  let fees = 25_000;
  let salary:f64=35_000.00;
 println!("fees is {} and salary is {}",fees,salary);
}

```

The output of the above code will be `fees is 25000 and salary is 35000`.

## Immutable

By default variables are immutable i.e.read only in Rust.In other words, the variable's value cannot be changed once a value is bound to a variable name.

Let us understand this with an example.

```rust
  fn main() {
  let fees = 25_000;
  println!("fees is {} ",fees);
  fees=35_000;
  println!("fees changed is {}",fees);
}

```

The output will be -

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

Variables are immutable by default.
Prefix the variable name with **mut** keyword to make it mutable. The value of a mutable variable can be changed.

The syntax for declaring a mutable variable is as shown below-

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

The output of the snippet is  given below-

```rust
  fees is 25000
  fees changed is 35000
```