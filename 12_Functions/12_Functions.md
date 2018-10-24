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

### Passing by Value

In the following example we have a variable `no` which is initally 5 and we are passing to 
```rust
fn main(){
     
     let  no:i32 = 5;
     mutate_no_to_zero(no);
     println!("The value of no is:{}",no);
}


fn mutate_no_to_zero(mut param_no: i32){
    param_no =param_no*0;
    println!("param_no value is :{}",param_no);
}

```
output is 

```rust
param_no value is :0
The value of no is:5
```

### Passing by Reference

The following program shows how to pass a number as reference to another function . The second function takes the `no` i32 as reference and modifies it to zero . Finally when we print the no
in main we the value will be mutated to zero.

```rust
fn main(){
     
     let mut no:i32 = 5;
     mutate_no_to_zero(&mut no);
     println!("The value of no is:{}",no);
}

fn mutate_no_to_zero(param_no:&mut i32){
    *param_no =0; //de reference
}

```

ouput will be `The value of no is:0` .


## Passing String 

```rust
 
  fn main(){
     
     let  name:String = String::from("TutorialsPoint");
     display(name);
    // println!("The value of name is:{}",name); //Error
}

fn display(param_name:String){
    
    println!("param_name value is :{}",param_name);
}

```