# Functions

Functions are the building blocks of readable, maintainable, and reusable code. A function is a set of statements to perform a specific task. Functions organize the program into logical blocks of code. Once defined, functions may be called to access code. This makes the code reusable. Moreover, functions make it easy to read and maintain the program’s code.

A function declaration tells the compiler about a function's name, return type, and parameters. A function definition provides the actual body of the function.

|S.No| Name | Description
|:----|:-----|:----------
| 1   | Defining a function | TA function definition specifies what and how a specific task would be done
| 2   | Calling or invoking a Function | A function must be called so as to execute it
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

The output will be-

`hello from function fn_hello`

## Returning value from a Function

Functions may also return a value along with control,back to the caller. Such functions are called as returning functions.

### Syntax

Either of the following syntax can be used to define a function with return type.

1. With return statement

```rust
// Syntax1
function function_name()  -> return_type {
   //statements
   return value;
}
```

2. Shorthand syntax without return statement

```rust
//Syntax2
function function_name()  -> return_type {
    value  //no semicolon means this value is returned
}
```

## Illustration

```rust
fn main(){
    println!("pi value is {}",get_pi());
}

fn get_pi()->f64{
    22.0/7.0
}

```

Output

```rust
 pi value is 3.142857142857143
```

## Function with Parameters

Parameters are a mechanism to pass values to functions. Parameters form a part of the function’s signature. The parameter values are passed to the function during its invocation. Unless explicitly specified, the number of values passed to a function must match the number of parameters defined.

Parameters can be passed to a function using one of the following techniques-

### Passing by Value

When a method is invoked, a new storage location is created for each value parameter.The values of the actual parameters are copied into them. Hence, the changes made to the parameter inside the invoked method have no effect on the argument.

In the following example we have a variable `no` which is initially 5 and we are passing to  `mutate_no_to_zero` which changes the value to zero.  After the function call when control returns back to main method the value will be the same.

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

output is:

```rust
param_no value is :0
The value of no is:5
```

### Passing by Reference

When you pass parameters by reference, unlike value parameters, a new storage location is not created for these parameters.The reference parameters represent the same memory location as the actual parameters that are supplied to the method.You can pass the reference parameters using the **&** keyword.

In the following example we have a variable `no` which is initially 5 and we are passing to  `mutate_no_to_zero` by reference which changes the value to zero. After the function call when control returns back to main method the value will be the zero.

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

The `*param_no` syntax is used to access value in the address passed.This is also known as *de referencing*

The ouput will be `The value of no is:0` .

## Passing String to function

 In this example we are passing string object to another function. Due to ownership feature of rust the owner of string object is `name` variable. During `display` function call, ownership of string object is moved to the method and after that it get invalidated. So we cannot use name variable after invoking display as shown in example.

```rust
  fn main(){
     let  name:String = String::from("TutorialsPoint");
     display(name); //cannot access name after display
    // println!("The value of name is:{}",name); //Error since name variable is invalidated
}

fn display(param_name:String){
    println!("param_name value is :{}",param_name);
}

```

output is :

`param_name value is :TutorialsPoint` 
