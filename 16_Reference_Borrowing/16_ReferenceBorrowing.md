# Borrowing

It is very inconvenient to pass the ownership of a variable to another function and then return the ownership. Rust supports a concept, borrowing, where the ownership of a value is transferred temporarily to an entity and then returned back to the original owner entity.

Consider the following-

```rust
  
fn main(){

    // a list of nos
    let v = vec![10,20,30];
    print_vector(v);
    println!("{}",v[0]); // this line gives error

}

fn print_vector(x:Vec<i32>){

    println!("Inside print_vector function {:?}",x);
}

   ```

The above example, the main function invokes a function `print_vector()` . A vector is passed as parameter to this function . The ownership of the vector is also passed to the `print_vector()` function from the `main()`. The above code will result in an error as shown below  when the `main()` function tries to access the vector `v`. 

   ```rust
  |     print_vector(v);
  |                  - value moved here
  |     println!("{}",v[0]);
  |                   ^ value used here after move
   ```

This is because, a variable or value can no longer be used by the function that originally owned it once ownership is tranfered to another funtion.

## What is Borrowing?

Borrowing is when a function transfers its control over a variable/value to another function temporarily,for a while. This is achieved by passing a reference `&` to the variable rather than passing the variable/value itself to the function.The ownership of the variable / value is transferred to the original owner of the variable after the function to which the control was passed to completes execution.

```rust

fn main(){
    // a list of nos
    let v = vec![10,20,30];
    print_vector(&v); // passing reference
    println!("Printing the value from main() v[0]={}",v[0]);

}

fn print_vector(x:&Vec<i32>){

    println!("Inside print_vector function {:?}",x);
}

```

output

```rust

Inside print_vector function [10, 20, 30]

Printing the value from main() v[0]=10

```

### &mut references

There’s a second kind of reference: `&mut` . A *mutable reference* allows you to mutate the resource you’re borrowing .In the below example we are passing string value to function `display` . The ownership is passed to the display method ,after executing display the ownership is invalidated.

```rust
  fn main(){
     let  name:String = String::from("TutorialsPoint");
     display(name); //cannot access name after display
     println!("The value of name is:{}",name); //line 4:Error since name variable is invalidated
}

fn display(param_name:String){
    println!("param_name value is :{}",param_name);
}

```

When we execute we get error at line 4 `^^^^ value borrowed here after move` . This means value is already moved so we cannot borrow.

In the below example we modified program to use **&** references .

```rust
fn main(){
     let  name:String = String::from("TutorialsPoint");
     display(&name); //cannot access name after display
     println!("The value of name is:{}",name);
}

fn display(param_name:&String){
    println!("param_name value is :{}",param_name);
}

```
output is :

```rust
param_name value is :TutorialsPoint
The value of name is:TutorialsPoint
```

Finally we modified program to use **&mut** references (mutable reference) to modify the passed value as below.

```rust

 fn main(){
     let  mut name:String = String::from("TutorialsPoint");
     display(&mut name); //cannot access name after display
     println!("The value of name is:{}",name);
}

fn display(param_name:&mut String){
    println!("param_name value is :{}",param_name);
    param_name.push_str(" Rocks"); //Modify the value passed
}

```

output:

```rust
param_name value is :TutorialsPoint
The value of name is:TutorialsPoint Rocks

```