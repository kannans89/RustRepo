# Reference and Borrowing

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

The above example, the main fuction invokes a function `print_vector()` . A vector is passed as parameter to this function . The ownership of the vector is also passed to the `print_vector()` function from the `main()`. The above code will result in an error as shown below  when the `main()` function tries to access the vector `v`. 

   ```rust
  |     print_vector(v);
  |                  - value moved here
  |     println!("{}",v[0]);
  |                   ^ value used here after move
   ```

This is because, a variable or value can no longer be used by the function that originally owned it once ownership is tranfered to another funtion.

## What is Borrowing?

Borrowing is when a function transfers its control over a variable/value to another function temporarily,for a while. This is achieved by passing a reference to the variable rather than passing the variable/value itself to the function.The ownership of the variable / value is transferred to the original owner of the variable after the function to which the control was passed to completes execution.

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

//appu: recheck
## Mutable References

Let us see the following code

```rust

fn main(){
      let mut a = 100;
      let b = &mut a;
      *b  +=2;
      println!("a={}",a);
      //you cannot use a untill you unborrow a from its control

}

```

The code will give your error *cannot borrow `a`  because it is also borrowed as mutable*
This ideally means *b* has borrowed value from *a* but it didn't return or release it after use.
To make this code work we can give a scope for variable *b*

```rust
fn main(){
      let mut a = 100;
      {
      let b = &mut a;
      *b  +=2;
      }
      println!("a={}",a);
      //you cannot use a untill you unborrow a from its control
}

```

so what is the rule protecting against data races

- you can have more than one reference to a resource , however
  there can only one mutable reference to a resource.

Imagine you are iterating across a mutable vector and during iteration you
are modifying the vector as shown below . This will give a compilation error in Rust as this result in undefined bhavior of the vector collection.
There by it brings in memory safety.This is one of the problems which plague other programming languages.

```rust
fn main(){

      let mut z = vec![10,20,30];

      for i in &z {
           print!("i={}\t",i);
           z.push(40);// compilation error
      }

```

### &mut references

passing string value to function

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


Mutable reference of string 

```rust

 fn main(){
     let  mut name:String = String::from("TutorialsPoint");
     display(&mut name); //cannot access name after display
     println!("The value of name is:{}",name); //Error since name variable is invalidated
}

fn display(param_name:&mut String){
    println!("param_name value is :{}",param_name);
    param_name.push_str(" Rocks");
}

```



