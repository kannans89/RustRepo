# Reference and Borrowing

### Borrowing for a while

Previous chapter we discussed about issues related to ownership.It is very
in convenient to pass control of a variable to another function and then return the value , ownership.So one of the notions that is supported by rust is called borrowing ( we can borrow ownership for a while).
   
   Let us look at how this works. 
  

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

   In the above code , We have main fuction inside this function we call another
   function `print_vector` , once we pass a vector to this function we are actually passing the ownership also . So once ownership is tranfered to that funtion the variable **v** can be  no longer used. 
   The owership is moved .
   The line `  println!("{}",v[0]);` gives an error as shown

   ```rust
  |     print_vector(v);
  |                  - value moved here
  |     println!("{}",v[0]);
  |                   ^ value used here after move
   ```

So what is borrowing ? Instead of giving over control to `print_vector`, we can let `print_vector` borrow vector `v` for a while. so we can change syntax from print_vector(v) to `print_vector(&v)` . Now the function will take the address or reference.

```rust

fn main(){
    // a list of nos
    let v = vec![10,20,30];
    print_vector(&v); // passing reference
    println!("{}",v[0]);

}

fn print_vector(x:&Vec<i32>){

    println!("Inside print_vector function {:?}",x);
}



```