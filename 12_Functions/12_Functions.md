# Functions

Functions are the building blocks of readable, maintainable, and reusable code. A function is a set of statements to perform a specific task. Functions organize the program into logical blocks of code. Once defined, functions may be called to access code. This makes the code reusable. Moreover, functions make it easy to read and maintain the program’s code.

A function declaration tells the compiler about a function's name, return type, and parameters. A function definition provides the actual body of the function.

|S.No| Name | Description
|:----|:-----|:----------
| 1   | Defining a function | TA function definition specifies what and how a specific task would be done
| 2   | Calling a Function | A function must be called so as to execute it
| 3   | Returning Functions | Functions may also return value along with control, back to the caller
| 4   | Parameterized Function | Parameters are a mechanism to pass values to functions.

## Syntax

```rust
 fn  function_name() {
   // function body
}

```

## Example

```rust
 fn main(){
     //calling a function
    fn_hello();
}

//Defining a function
fn  fn_hello(){
    println!("hello from function fn_hello ");
}

```

output `hello from function fn_hello`

## Returning value from a Function

Functions may also return value along with control,back to the caller. Such functions are called as returning functions.

### Syntax

```rust
// Syntax1
function function_name()  -> return_type {
   //statements
   return value;
}
```

Second syntax with no return keyword and semicolon.

```rust
//Syntax2
function function_name()  -> return_type {
    value  //no semicolon ,shows return
}
```

## Example

```rust
fn main(){
    println!("pi value is {}",get_pi());
}

fn get_pi()->f64{
    22.0/7.0
}

```

output is shown below

```rust
 pi value is 3.142857142857143
```

## Function with Parameters

Parameters are a mechanism to pass values to functions. Parameters form a part of the function’s signature. The parameter values are passed to the function during its invocation. Unless explicitly specified, the number of values passed to a function must match the number of parameters defined.