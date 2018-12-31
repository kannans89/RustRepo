# Iterator and Closure

# Iterators


Iterators gives a way of processing a series of elements.An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished.Iterators implement the Iterator trait.
The standard library defines an Iterator as a trait as shown.

```rust
trait Iterator {
     type Item;
     fn next(&mut self) -> Option<Self::Item>
}

```

The associated type `Item` is returned when we call next() method on an iterator.The Iterator trait is used to implement iterators over collections such as arrays,vector,maps.

To access iterator from a collection we can use `iter()` method. Example is shown below

```rust
 fn main() {
    let a = [10,20,30];
    let mut iter = a.iter();
    println!("{:?}",iter);

    // a call to next returns next value

    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
}

```

In the above program we are accessing the iterator and manully calling the next method.Since the next method is returning option type,if no data is found it returns None as shown in output

```rust
Iter([10, 20, 30])
Some(10)
Some(20)
Some(30)
None
```

If a collection like array or Vect implements Iterator trait then it can be traversed using the  `for...in` syntax as shown.

```rust
fn main() {
    let a = [10,20,30];
    let  iter = a.iter();
    for data in iter{
        print!("{}\t",data);
    }
}
```

ouput is `10	20	30`


There are 3 methods which generally crate iterators.Given x is a collection of some type T:

Sr No |  methods    | description|
|:-----|:-------|:---------|
|1|x.iter()|gives an iterator over &T|
|2|x.into_iter()|gives an iterator over T|
|3|x.iter_mut()|gives an iterator over &mut T|


### Illustraion:for and iter()

This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.

```rust
fn main() {
     let names = vec!["Kannan", "Mohtashim", "Kiran"];

    for name in names.iter() {
        match name {
            &"Mohtashim" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("{:?}",names);
}


```

output

```rust
Hello Kannan
There is a rustacean among us!
Hello Kiran
["Kannan", "Mohtashim", "Kiran"]


```

### Illustraion:for and into_iter()

This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.

```rust
fn main(){
    let names = vec!["Kannan", "Mohtashim", "Kiran"];

    for name in names.into_iter() {
        match name {
            "Mohtashim" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

   // println!("{:?}",names); //Error:Cannot access after ownership move
}

```

ouput: 

```rust
Hello Kannan
There is a rustacean among us!
Hello Kiran
```

### Illustraion: for and iter_mut()

This mutably borrows each element of the collection, allowing for the collection to be modified in place.

```rust
fn main() {
      let mut names = vec!["Kannan", "Mohtashim", "Kiran"];

    for name in names.iter_mut() {
        match name {
            &mut "Mohtashim" => println!("There is a rustacean among us!"),
            _ =>  println!("Hello {}", name),
        }
    
    }

    println!("{:?}",names);
}


```

output:

```rust
Hello Kannan
There is a rustacean among us!
Hello Kiran
["Kannan", "Mohtashim", "Kiran"]

```


Iterators are lazy , means evaluation are not done util the results are actully required. This means you can have an iterator of 1 to 100 million and you wont exhaust your memory.

## Closure

 Closures are essentially functions that can be defined inline and close over variables in their scope.A closure is a function that closes over or captures its environment.This means it is defined in line with other code and can access bindings declared in that code.closures are anonymous and their types cannot be named.

  Syntax

  ```rust
     let closure_function = |parameter| {
         //logic
     }

     closure_function('somedavalue');//invoking
  ```
  
A closure is written with two vertical bars surrounding its arguments.
A closure implements one of the **Fn** family of traits , meaning it can be invoked with **()** syntax like a function.

### Illustraion

 Here we have a function within a funciton as closure named `is_evn`.

 ```rust
  fn main(){

    let is_even = |x| {
        x%2==0
    };
   let no = 13;
    println!("{} is even ? {}",no,is_even(no));

}


 ```

output:`13 is even ? false`

### Illustraion :closing over variable

Here variable val is declared outside of the closure function but can be accessed with in the scope.

 ```rust
 fn main(){
    let val = 10;
    let closure2 = |x| {
        x + val
    };
    println!("{}",closure2(2));
}

 ```

 We cannot give a type to the closure variable used  `closure2` variable its type is always anonymous type or unspeakable type.

 output:`12`
