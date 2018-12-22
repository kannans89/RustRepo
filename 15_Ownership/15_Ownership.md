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

## Different ways to move ownership

1. Assigning value from one variable to another variable
2. Passing value  to function
3. Returning value from function


## More on Ownership

the key selling point of rust as a language is its memory safety.Memory safety is achieved by tight control on who can use what and when restrictions.

We were  unable to use a variable after a closure took over them.
Let us suppose that we have a vector called v as shown

```rust
fn main(){
  let v = vec![1,2,3]; // vector v owns the object in heap
  
 //only a single variable owns the heap memory at any given time
  let v2 = v;  // here two variables owns heap value,
  //two pointers to the same content is not allowed in rust

 //Rust is very smart in terms of memory access ,so it detects a race condition
//as two variables point to same heap 
}

```

The idea of only one variable binds to a resource , either v binds to resource or v2 binds to the resource

let v2=v;// now v2 is owner of the resource

since v2 is owner of resource , v is no longer availble.

so if you run the program above we get errr ,` value used here after move` on the println macro line.

```rust
fn main(){
 
  let v = vec![1,2,3]; // vector v owns the object in heap

 let v2 = v;  // moves ownership to v2  
 
 println!("{:?}",v);
}

```

## what is move ??

It means the ownership is moved from v to v2 as we assign v2=v , it also invalidates v after the move.

this happens when we pass an object in heap to a closure or function as shown.

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

One work around of this case is let the function return the owned object as shwon below

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

If we consider the case of primitive types, contents from one variable is copied to another. So there is no ownership move happening .let us see an example 

```rust

fn main(){
 
  let u1 = 10;
  let u2=u1; // u1 value copied(not moved) to u2
  
  println!("u1 = {}",u1);
}

```

output will be : 10

This is because of an i32 variable needs less resources than an object which is allocated in heap.

## Converting primitive to Object with Box types

 If we want the primitives behave like Ojbect types we can box it and in that case ownership move will occur .Let us see an example.

```rust
fn main(){
  let u1 = Box::new(10);
  let u2=u1; // value has been moved
  
  println!("u1 = {}",u1); //Error: value used here after move
}
```

The reason for error is same as we assign u1 to u2 .After assignment u1 will be invalidated.
