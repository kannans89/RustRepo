# Ownership

Memory for a program is allocated in :

- Stack or
- Heap
  
## Stack

A stack follows a last in first out order.Stack stores data values for which the size is known at compile time.For example, a variable of fixed size **i32** is a candidate for stack allocation. Its size is known at compile time.

All scalar types can be stored in stack as the size is fixed.

Consider an example of a string which is assigned a value at runtime.The exact size of such a string cannot be determined at compile time .So it is not a candidate for stack allocation but for heap allocation.

## Heap

The heap memory stores data values for which size is to unknown at compile time.It is used to store dynamic data. Simply put, a heap memory is allocated to data values that may change throughout the life cycle of the program.The heap is some part of memory which is less organized as compared to stack. 

//appu: check this 

When an object is created it is allocated space in memory and its reference or pointer is returned to stack variable.

## What is Ownership

- Each value in Rust has a variable that is called **owner** of the value.Every data stored in Rust will have an owner associated with it.
 
  For example: In the syntax `let age=30` , *age* is the owner of the value  *30*.

- Each data can have only one owner at a time.
- Two variables cannot point to the same memory location.They will be always pointing to different memory location.

## Transferring ownership

The ownership of value can be transferred by -

1. Assigning value from one variable to another variable
2. Passing value  to a function
3. Returning value from a function


## More on Ownership

The key selling point of rust as a language is its memory safety.Memory safety is achieved by tight control on who can use what and when restrictions.

Consider the following snippet-

A vector called v is declared as shown below-

```rust
fn main(){
  let v = vec![1,2,3]; // vector v owns the object in heap
  
 //only a single variable owns the heap memory at any given time
  let v2 = v;  // here two variables owns heap value,
  //two pointers to the same content is not allowed in rust

 //Rust is very smart in terms of memory access ,so it detects a race condition
//as two variables point to same heap 

 println!("{:?}",v);

}

```

The idea of ownership is that only one variable binds to a resource , either v binds to resource or v2 binds to the resource.
The above example throws an error ,` value used here after move`,since v2 is the owner of resource , v is no longer availble.
It means the ownership is moved from v to v2 as we assign v2=v , it also invalidates v after the move.
This also happens when we pass an object in heap to a closure or function as shown.

```rust
fn main(){
  let v = vec![1,2,3]; // vector v owns the object in heap
 let v2 = v;  // moves ownership to v2
 display(v2); // v2 is moved to display and v2 is invalidated
  println!("In main {:?}",v2); //v2 is No longer usable here
}

fn display(v:Vec<i32>){

     println!("inside display {:?}",v);
}

```

One work around for this is let the function return the owned object as shown below

```rust
 fn main(){
  let v = vec![1,2,3]; // vector v owns the object in heap
 let v2 = v;  // moves ownership to v2
  let v2_return =display(v2);
  println!("In main {:?}",v2_return);

  
}

fn display(v:Vec<i32>)->Vec<i32>{ // returning same vector

     println!("inside display {:?}",v);
     v
}

```

The above is true for complex data types. In case of primitive types, contents from one variable is copied to another. So there is no ownership move happening . This is because of a primitive variable needs less resources than an object.Consider the following example-

```rust

fn main(){
 
  let u1 = 10;
  let u2=u1; // u1 value copied(not moved) to u2
  
  println!("u1 = {}",u1);
}

```

The output will be : 10
