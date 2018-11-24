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

So what is borrowing ? Instead of giving over control to `print_vector`, we can let `print_vector` borrow vector `v` for a while. so we can change syntax from print_vector(v) to `print_vector(&v)` . Now the function will take the address or reference. The advantage of this approach is that it is technically ok to access the vector **v** after the function call.

```rust

fn main(){
    // a list of nos
    let v = vec![10,20,30];
    print_vector(&v); // passing reference
    println!("v[0]={}",v[0]);

}

fn print_vector(x:&Vec<i32>){

    println!("Inside print_vector function {:?}",x);
}



```

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

The code will give your error *cannot borrow `a` as immutable because it is also borrowed as mutable*
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