# Ownership

Let us understand how memory is allocated for program in Rust.

- Stack
- Heap
  
## Stack

stack follows a last in first out order.Stack stores data whose size is known at compile time.For example lets say a variable of fixed size **i32** is a candidate for stack allocation by rust. Its size is known at compile time.All scalar types can be stored in stack as the size is fixed.

Lets take an example of a string whose values and size are decided at run time.It is difficult to know the exact size of a string at compile time .So it is not a candidate for stack allocation but for heap allocation.

## Heap

 Heap is place where rust allocates data whose size is to unknown at compile time.It may by dynamic means data can change throughout the life cycle of the program.The heap is some part of memory which is less organized compared to stack. When an object is created it is allocated space in memory and its reference or pointer is returned to stack variable.

## What is Ownership

- Each value in Rust has a variable that is called **owner** of the value.Every data stored in rust will have an owner associated with it.For example if `let age=30` then *age* is the owner of date *30*.

- Each data can have only one owner at a time.Two variables cannot point to the same memory location.They will be always pointing to different memory location.

For example see the following program

```rust
   fn main(){
    let name = String::from("kannan");
    let another_name=name;
    //println!("name is:{} ",name);
    println!("another name is :{}",another_name);
}

```

## Different ways to move ownership

1. Assigning value from one variable to another variable
2. Passing value  to function
3. Returning value from function

