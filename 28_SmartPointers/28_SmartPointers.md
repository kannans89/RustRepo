# Smart Pointers

Rust allocates everything on the stack by default. To store things on the heap you have to do so explicitly ,usually by wrapping them in smart pointers like `Box`.Types like Vec and String implicitly does heap allocation..The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement following traits


|Sr.No|trait name|package|description
|:---|:---|:---|:-------|
|1|Deref|std::ops::Deref|Used for immutable dereferencing operations, like *v.
|2|Drop|std::ops::Drop|Used to run some code when a value goes out of scope. This is sometimes called a *destructor*

This chapter discusses on  **Box** smart pointer and how to create a custom smart pointer like Box.

## Box<T>

The most straight forward smart pointer is a box, whose type is written as Box<T>. A Box allows you to store data on the heap rather than the stack. The stack contains the pointer to the heap data.A Box doesn’t have performance overhead, other than storing their data on the heap.

Let us see how to use a box to store an i32 value on the heap.

```rust

    fn main() {
        let var_i32 = 5; //stack
        let b = Box::new(var_i32); //heap
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

### Illustraion : Deref Trait

 The Deref trait, provided by the standard library, requires us to implement one method named *deref* that borrows *self* and returns a reference to the inner data

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> { // Generic structure with static method new
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

    fn deref(&self) -> &T {
      &self.0  //returns data
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x); // calling static method

    println!("5==x is {}",5==x);
    println!("5==*y is {}",5==*y); // dereferencing y
     println!("x==*y is {}",x==*y);//dereferencing y
}


```

In the above example we creating a structure `MyBox` which is generic type.It implements the trait `Deref`,because of this trait we can access heap values wrapped by `y` using `*y`.

output is shown below

```rust
5==x is true
5==*y is true
x==*y is true
```

### Illustraion : Drop Trait

The Drop trait has `drop` method . This method will be called when a structure which implemented this trait goes out of scope .In some languages, the programmer must call code to free memory or resources every time they finish using an instance of a smart pointer. In Rust, you can achieve automatic memory de allocation using Drop trait.

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

In the above example the drop method will be called twice as we are creating two objects in the heap.

output:

```rust
dropping MyBox object from memory
dropping MyBox object from memory 
```