# Smart Pointers


Rust is very much like C++, it will put everything on the stack by default. To store things on the heap you have to do so explicitly (usually by wrapping them in smart pointers like Box or Rc).
Note however that (also as in C++) some types may "implicitly" perform heap allocations e.g. String or Vec


============================================================
A pointer contains the address of a variable in the memory.The most common kind of pointer in Rust is a reference.References are indicated by the `&` symbol and borrow the value they point to. They don’t have any special capabilities other than referring to data. Also, they don’t have any overhead and are the kind of pointer we use most often.

Smart pointers are data structures that not only act like a pointer but also have additional metadata and capabilities. The difference between a pointer and smart pointers is that pointers only borrow data; in contrast, in many cases, smart pointers own the data they point to.
The types String and Vec<T> are examples of smart pointers.The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the *Deref* and *Drop* traits

This chapter discusses about one of the smart pointers **Box** and how to create custom smart pointer.
<!--
Common smart pointers include
1. Box<T>  Allocating values on the heap
2. Rc<T>   A reference counting type that enables multiple ownership
3. RefCell<T> A type that enforces the borrowing rules at runtime instead of compile time-->

## Box<T>

The most straight forward smart pointer is a box, whose type is written as Box<T>. A Box allows you to store data on the heap rather than the stack. The stack contains the pointer to the heap data.A Box doesn’t have performance overhead, other than storing their data on the heap.

Let us see how to use a box to store an i32 value on the heap.

```rust

    fn main() {
        let var_i32 = 5;
        let b = Box::new(var_i32);
        println!("b = {}", b);
    }

```

Output:

```rust

b = 5

```

In order to access a value pointed by a variable,use deferencing.The `*` is used as a dereference operator.Let us see how to use dereference with Box.

The following example shows a value type `x` is boxed to an object type.

```rust
 fn main() {
    let x = 5; //value type variable
    let y = Box::new(x); //y points to a new value 5 in the heap

    println!("{}",5==x);  
    println!("{}",5==*y); //dereferencing y
    }

```
The variable x is a value-type with the value 5. So, the expression `5==x` will return true. Variable `y` points to the heap and to access the value in heap we need to dereference using `*y`. ``*y` returns value 5. So, the expression `5==*y` returns true.

Output:

```rust
true
true

```

//appu: add content for Rc or one more example 

## Defining a Custom Smart Pointer

While Rust provides an inbuilt smart pointer, it also allows us to create custom smart pointers.

The Box<T> type is  defined as a tuple struct  with one element.So let us define a MyBox<T> type in same way as shown. Also we need to add a function with name new() similar to Box<T>.

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}

```

 The MyBox::new function takes one parameter of type T  and returns a MyBox instance that holds the value passed in.If we create a MyBox variable and check equality using dereference operator as below will give error.*error[E0614]: type `MyBox<{integer}>` cannot be dereferenced*

 ```rust
  fn main() {
    let x = 5;
    let y = MyBox::new(x);

    println!("{}",5==x);
    println!("{}",5==*y); // ILLEGAL
}

 ```

 Our MyBox<T> type can’t be dereferenced because we haven’t implemented that ability on our type. To enable dereferencing with the * operator, we implement the Deref trait.

 ## Deref Trait Implmentation

 The Deref trait, provided by the standard library, requires us to implement one method named *deref* that borrows *self* and returns a reference to the inner data
So the full implementation is as shwon below

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

    fn deref(&self) -> &T {
      &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    println!("5==x is {}",5==x);
    println!("5==*y is {}",5==*y);
     println!("x==*y is {}",x==*y);
}


```

output is shown below

```rust
5==x is true
5==*y is true
x==*y is true
```

## Drop Trait

Drop is similar to destrcutor.

In some languages, the programmer must call code to free memory or resources every time they finish using an instance of a smart pointer. If they forget, the system might become overloaded and crash. In Rust, you can specify that a particular bit of code be run whenever a value goes out of scope, and the compiler will insert this code automatically. As a result, you don’t need to be careful about placing cleanup code everywhere in a program that an instance of a particular type is finished with—you still won’t leak resources!


```rust

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

    fn deref(&self) -> &T {
      &self.0
    }
}


impl<T> Drop for MyBox<T>{
   fn drop(&mut self){
    
       println!("dropping MyBox object from memory ");
   }    
}

fn main() {
    let x = 50;
    MyBox::new(x);
    MyBox::new("Hello");
    
}
```

output:

```rust
dropping MyBox object from memory
dropping MyBox object from memory 
```