# Iterator and Closure

# Iterators

An iterator helps to iterate over a collection of values such as arrays, vectors,maps etc.Iterators implement the Iterator trait that is  defined in the Rust standard library.
The `iter()` method returns an iterator object of the collection. Values in an iterator object are called items. The `next()` method of the iterator can be used to traverse through the items. The `next` method returns a value None when it reaches the end of the collection.

The following example uses an iterator to read values from an array.

```rust
 fn main() {
   
    //declare an array
    let a = [10,20,30];
    
    let mut iter = a.iter(); // fetch an iterator object for the array
    println!("{:?}",iter);

    //fetch individual values from the iterator object
     println!("{:?}",iter.next());
     println!("{:?}",iter.next());
     println!("{:?}",iter.next());
     println!("{:?}",iter.next());
}

```

```rust
Iter([10, 20, 30])
Some(10)
Some(20)
Some(30)
None
```

If a collection like array or Vector implements Iterator trait then it can be traversed using the  `for...in` syntax as shown below.

```rust
fn main() {
    let a = [10,20,30];
    let  iter = a.iter();
    for data in iter{
        print!("{}\t",data);
    }
}
```

Output: `10	20	30`


The following 3 methods return an iterator object for a collection.

//appu: change description

Sr No |  methods    | description|
|:-----|:-------|:---------|
|1|iter()|gives an iterator over &T|
|2|into_iter()|gives an iterator over T|
|3|iter_mut()|gives an iterator over &mut T|


### Illustration:iter()
The iter() function uses the concept of borrowing. It returns a reference to each element of the collection,leaving the collection untouched and available for reuse after the loop.

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

Output

```rust
Hello Kannan
There is a rustacean among us!
Hello Kiran
["Kannan", "Mohtashim", "Kiran"]


```

### Illustration:into_iter()

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
