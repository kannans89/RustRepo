# Smart Pointers

A pointer is a general concept for a variable that contains an address in memory.
This address refers to some data.The most common kind of pointer in Rust is a reference.References are indicated by the & symbol and borrow the value they point to. They don’t have any special capabilities other than referring to data. Also, they don’t have any overhead and are the kind of pointer we use most often.

Smart pointers, on the other hand, are data structures that not only act like a pointer but also have additional metadata and capabilities. In Rust, the different smart pointers defined in the standard library provide functionality beyond that provided by references.

An additional difference between references and smart pointers is that references are pointers that only borrow data; in contrast, in many cases, smart pointers own the data they point to.

Example of smart pointer we already discussed include String and Vec<T> . Both these types count as smart pointers because they own some memory and allow you to manipulate it. They also have metadata (such as their capacity) and extra capabilities or guarantees (such as with String ensuring its data will always be valid UTF-8).
he characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the *Deref* and Drop traits


Common smart pointers include
1. Box<T>  Allocating values on the heap
2. Rc<T>   A reference counting type that enables multiple ownership
3. RefCell<T> A type that enforces the borrowing rules at runtime instead of compile time

## Box<T>

The most straightforward smart pointer is a box, whose type is written Box<T>. Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.

Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack. But they don’t have many extra capabilities either. You’ll use them most often in these situations:

Let us see how to use a box to store an i32 value on the heap.

```rust

    fn main() {
        let var_i32 = 5;
        let b = Box::new(var_i32);
        println!("b = {}", b);
    }

```

Let us see how to use dereference with Box

```rust
 fn main() {
    let x = 5;
    let y = Box::new(x);

    println!("{}",5==x);
    println!("{}",5==*y);
}

```

## Defining Our Own Smart Pointer

Let’s build a smart pointer similar to the Box<T> type provided by the standard library to experience how smart pointers behave differently than references by default. Then we’ll look at how to add the ability to use the dereference operator.

The Box<T> type is  defined as a tuple struct  with one element.So let us define a ProBox<T> type in same way as shown. Also we need to add a function with name new() similar to Box<T>.

```rust
struct ProBox<T>(T);

impl<T> ProBox<T> {
    fn new(x:T)->ProBox<T>{
        ProBox(x)
    }
}

```

 The ProBox::new function takes one parameter of type T  and returns a ProBox instance that holds the value passed in.If we create a ProBox variable and check equality using dereference operator as below will give error.*error[E0614]: type `ProBox<{integer}>` cannot be dereferenced*

 ```rust
  fn main() {
    let x = 5;
    let y = ProBox::new(x);

    println!("{}",5==x);
    println!("{}",5==*y); // ILLEGAL
}

 ```

 Our ProBox<T> type can’t be dereferenced because we haven’t implemented that ability on our type. To enable dereferencing with the * operator, we implement the Deref trait.

 ## Deref Trait Implmentation

 The Deref trait, provided by the standard library, requires us to implement one method named *deref* that borrows *self* and returns a reference to the inner data
So the full implementation is as shwon below

```rust
use std::ops::Deref;

struct ProBox<T>(T);

impl<T> ProBox<T> {
    fn new(x:T)->ProBox<T>{
        ProBox(x)
    }
}

impl<T> Deref for ProBox<T> {
  type Target = T;

    fn deref(&self) -> &T {
      &self.0
    }
}

fn main() {
    let x = 5;
    let y = ProBox::new(x);

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

struct ProBox<T>(T);

impl<T> ProBox<T> {
    fn new(x:T)->ProBox<T>{
        ProBox(x)
    }
}

impl<T> Deref for ProBox<T> {
  type Target = T;

    fn deref(&self) -> &T {
      &self.0
    }
}


impl<T> Drop for ProBox<T>{
   fn drop(&mut self){
    
       println!("dropping Probox object from memory ");
   }    
}

fn main() {
    let x = 50;
    ProBox::new(x);
    ProBox::new("Hello");
    
}
```