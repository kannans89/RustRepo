# Iterator and Closure

Iterator allows the for...in ..syntax to work.Lots of collections implement this trait like

- Vect
- HashMap
- LinkedList

Functional programming fundamental is functions. We will look at now how to use functions as values. Closures are essentially functions that can be defined inline and close over variables in their scope.

## Closure

 A closure is a function that closes over or captures its environment.This means it is defined in line with other code and can access bindings declared in that code.closures are anonymous and their types cannot be named.

  Syntax
    - A closure is written with two vertical bars surrounding its arguments.
    - A closure implements one of the **Fn** family of traits , meaning it can be called with **()** syntax like a function.


 ```rust
  fn main(){

    let is_even = |x| {
        x%2==0
    };
   let no = 13;
    println!("{} is even ? {}",no,is_even(no));

}


 ```

 closing over variable  example , here variable val is declared outside of the closure function but can be accessed with in the scope.

 ```rust
 fn main(){
    let val = 10;
    let closure2 = |x| {
        x + val
    };
    println!("{}",closure2(2));
}

 ```

 We cannot give a type to the closure variable use `closure2` variable its type is always anonymous type or unspeakable type.To return it or accept it as a function we must use generics or a dynamic trait object.Let us see an example

# Iterators

Iterators are types which implement the Iterator trait,requiring a .next() method , this has an associated type, which further specifies what that .next() method produces.

There are 3 methods which generally crate iterators.Given x is a collection of some type T:
- x.into_iter() gives an iterator over T
- x.iter_mut() gives an iterator over &mut T
- x.iter() gives an iterator over &T

The next() method returns an **Option**

Let us see an example , if the iteratio returns None that means the iteration is over.

```rust
fn main(){

    let mut iterator = (1..5).into_iter();
    println!("{:?}",iterator.next()); // Some(1)
    println!("{:?}",iterator.next()); // Some(2)
    println!("{:?}",iterator.next()); // Some(3)
    println!("{:?}",iterator.next()); // Some(4)
    println!("{:?}",iterator.next()); // None
    println!("{:?}",iterator.next()); // None
}

```

### take() method

The skip() and take() methods allow traversing forward along iterators.
skip() returns nothing , while take() returns an iterator over the number of
elements specified.

skip method example

```rust
fn main() {
   let mut iterator = (1..10).into_iter();
    for arg in iterator.skip(5) {
     println!("{:?}",arg);
   }
}

```

take() method example

```rust
 fn main() {
       let mut iterator = (1..10).into_iter();
    
    let mut taken = iterator.take(2); 
    println!("{:?}",taken.next()); // Some(1)
    println!("{:?}",taken.next()); // Some(2)
    println!("{:?}",taken.next()); //None
  
 }

```

enumerate() method adds indices to the elements iterated over.This is especially useful when iterating over slices,vectors and arrays.

example
```rust
fn main() {
    let mut iterator = vec!["A","B","C"].into_iter();
    let mut enumerated = iterator.enumerate();

    println!("{:?}",enumerated.next());
    println!("{:?}",enumerated.next());
    println!("{:?}",enumerated.next());
}

```

Iterators are lazy , means evaluation are not done util the results are actully required. This means you can have an iterator of 1 to 100 million and you wont exhaus your memeory.

The collect() method turns an iterator into a vector or some other collection that implements the Formlter trait.