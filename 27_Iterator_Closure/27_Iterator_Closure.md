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

Output :

```rust
Iter([10, 20, 30])
Some(10)
Some(20)
Some(30)
None
```

If a collection like array or Vector implements Iterator trait then it can be traversed using the  `for...in` syntax as shown below-

```rust
fn main() {
    let a = [10,20,30];
    let  iter = a.iter();
    for data in iter{
        print!("{}\t",data);
    }
}
```

Output: `10 20 30`

The following 3 methods return an iterator object from a collection,where T represents the elements in a collection.

Sr No |  methods    | description|
|:-----|:-------|:---------|
|1|iter()|gives an iterator over &T(reference to T)|
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

    println!("{:?}",names); // reusing the collection after iteration
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

This function uses the concept of ownership. It moves values in the collection into an iter object i.e. it consumes the collection.ce the collection has been consumed it is no longer available for reuse.

```rust
fn main(){
    let names = vec!["Kannan", "Mohtashim", "Kiran"];

    for name in names.into_iter() {
        match name {
            "Mohtashim" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

         // cannot  reuse the collection after iteration
   // println!("{:?}",names); //Error:Cannot access after ownership move
}

```

Output:

```rust
Hello Kannan
There is a rustacean among us!
Hello Kiran
```

### Illustration: for and iter_mut()

This function is similar to the `iter()` function.However, this function can modify elements within the collection.

```rust
fn main() {
      let mut names = vec!["Kannan", "Mohtashim", "Kiran"];

    for name in names.iter_mut() {
        match name {
            &mut "Mohtashim" => println!("There is a rustacean among us!"),
            _ =>  println!("Hello {}", name),
        }
    
    }

    println!("{:?}",names);//// reusing the collection after iteration
}


```

Output:

```rust
Hello Kannan
There is a rustacean among us!
Hello Kiran
["Kannan", "Mohtashim", "Kiran"]

```

## Closure
Closure refers to a function within another function.These are anonymous functions,i.e. functions without a name.
Closure can be used to assign a function to a variable. This allows a program to pass a function as a parameter to other functions.Closure is also known as an inline  function.Variables in the outer function can be accessed by inline functions.

Syntax : Defining a Closure

A closure definition may optionally have parameters.Parameters are enclosed within two vertical bars.

  ```rust
     let closure_function = |parameter| {
         //logic
     }
```

Syntax: Invoking a Closure
Closure implements **Fn** traits.So, it can be invoked with **()** syntax. 
```
     closure_function(parameter);//invoking
 ```
  

### Illustration

 The following example defines a closure `is_even` within the function `main()`. The closure returns true if a number is even and returns false if the number is odd.

 ```rust
  fn main(){

    let is_even = |x| {
        x%2==0
    };
   let no = 13;
    println!("{} is even ? {}",no,is_even(no));

}

 ```

Output:`13 is even ? false`

### Illustration

 ```rust
 fn main(){
    let val = 10; // declared outside
    let closure2 = |x| {
        x + val           //inner function accessing outer fn variable
    };
    println!("{}",closure2(2));
}

 ```

The `main()` function declares a variable `val` and a closure. The closure accesses the variable declared in the outer function `main()`.

 Output:`12`
