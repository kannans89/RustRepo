# Constant

Constants are variables whose value cannot be changed once assigned.If you declare a constant variable then there is no way its value changes.The keyword for using constants is **const** .Following is the syntax to create a constant.

```rust
    const VARIABLE_NAME:dataType=value;
```

Rust’s naming convention for constants is to use all uppercase with underscores between words.Note that there is no **let** keyword used. Following shows an example.

```rust
 fn main() {

 const USER_LIMIT:i32=100;
 const PI:f32 = 3.14;

 println!("user limit is {}",USER_LIMIT);
 println!("pi value is {}",PI);

}
```

## Constants vs Variables

- Constants are declared using **const** keyword and variables using **let**

- Data type of variable is at time of declaration is optional but for constants the type must be annotated.This means `const USER_LIMIT=100` will give error.

- A variable  declared using **let** keyword is by default immutable. But you have an option to make it mutable using **mut** keyword.

- Constants may be set only to a constant expression, not to the result of a function call or any other value that could only be computed at runtime

- Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of the code need to know about.

Let us see an example

The following program is illegal:
It is so because arrays must be of a length known at compile time, here variable "N" initial value could be only determined at runtime, and so it is not allowed to use variables to specify the size of an array as shown below

```rust
fn main() {

let N: usize = 20;
let arr = [0; N]; //Error: non-constant used with constant

print!("{}",arr[10])

    
}

```

But the following program is valid:

```rust
 fn main() {

const N: usize = 20;
let arr = [0; N];

print!("{}",arr[10])

    
}
```

The "const" keyword allows us to declare an identifier having a value defined at compile time, and of course no more changeable at runtime. In its declaration, it is required to specify its type.


## Shadowing of variables and constants

 Rust allows programmers to declare a new variable with the same name as a previous variable, and the new variable shadows the previous variable.
 Let us understand this with an example.

 ```rust
fn main() {
let salary = 100.00;
let salary = 1.50 * salary; // reads first salary
println!("salary after hike is :{}",salary);

}
```

From the above example lets take variable salary , its value is 100.00 floating value . Its is then multiplied and assigned to another variable of same name. This is allowed in Rust.  

Since salary is immutable or readonly you can only read the contents of the variable. Second variable which shadows or hides the first variable while displaying output.

output is : `salary after hike is :150`

Let us take another example where a string variable is used with a name assigned . Now we are shadowing the string variable to an integer type. Changing the type while shadowing is allowed in Rust as shown below.

```rust
fn main() {

 let name="Mohtashim";
 let name= name.len();
 println!("name changed to integer : {}",name);
}
```

output is `name changed to integer : 9`

Constants on the other hand cannot be shadowed if the same program is executed with constants as shown below , will giver error

```rust
    fn main() {

 const NAME:&str="Mohtashim";
 const NAME:usize= name.len(); //Error : `name` already defined
 println!("name changed to integer : {}",name);
}


```
