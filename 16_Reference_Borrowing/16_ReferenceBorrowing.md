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

Output

```rust

Inside print_vector function [10, 20, 30]

Printing the value from main() v[0]=10

```

## Mutable References

A function can modify a borrowed resource by using a *mutable reference* to such resource. A mutable reference is prefixed with `&mut`. Mutable references can operate only on mutable variables.

### Illustration : Mutating an integer reference

```rust
 fn add_one(e: &mut i32) {
   *e+= 1;
}

fn main() {
   let mut i = 3;
   add_one(&mut i);
   println!("{}", i);
}

```
The `main()` function  declares a mutable integer variable `i `and passes a mutable reference of `i` to the add_one(). The add_one() increments the value of the variable `i` by one.
 
### Illustration : Mutating a string reference

```rust

 fn main(){
     let  mut name:String = String::from("TutorialsPoint"); 
     display(&mut name); //pass a mutable reference of name
     println!("The value of name after modification is:{}",name);
}

fn display(param_name:&mut String){
    println!("param_name value is :{}",param_name);
    param_name.push_str(" Rocks"); //Modify the actual string,name
}

```
The `main()` function passes a mutable reference of the variable `name` to the `display()`function. The display function appends an additional string to the original `name` variable.

Output

```rust
param_name value is :TutorialsPoint
The value of name after modification is:TutorialsPoint Rocks

```
