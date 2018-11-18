# String

There are two main string types in Rust.

1. String Literal (&str) :  
2. String Collection(String) :

## String Literal

String literals are also known as string slices.This uses borrowing concept.
String literals are a list of characters which are hardcoded into a variable. For example `let company="TutorialsPoint"` . String literals are found in module
*std::str**  is part of core language. This is immutable.

## String Collection

The String type is provided in Standard Library , not part of core language.It is code as a public structure as shown `pub struct String`.
Uses ownership concept. String is a growable collection ,  it is mutable and UTF-8 encoded type.

### Syntax

 To create a string you can use two ways
 1. String::new()
   This creates an empty string
 2. String::from()
    This creates a string with some default value in from() method.

```rust
fn main(){
    let empty_string = String::new();
    println!("length is {}",empty_string.len());

    let content_string = String::from("TutorialsPoint");
    println!("length is {}",content_string.len());
}

```

output is as shown below

```rust
length is 0
length is 14

```

### Common methods in String is

|Sr No |  method | signature  | description
|:----:|:----------|:----|:-----------------|
| 1    | new()     | pub const fn new() -> String|Creates a new empty String.
| 2    | as_str()  | pub fn as_str(&self) -> &str  | Extracts a string slice containing the entire string.
|3    | push()     |pub fn push(&mut self, ch: char) |Appends the given char to the end of this String.
| 4    | push_str() |pub fn push_str(&mut self, string: &str)   | Appends a given string slice onto the end of this String.
| 5    | len()     |pub fn len(&self) -> usize |Returns the length of this String, in bytes.
| 6   | chars()     |pub fn chars(&self) -> Chars |Returns an iterator over the chars of a string slice.

<!-- TODO: clone(), -->

```rust
fn main() {
    let mut name=String::from("Tutorials");
    name.push_str(" Point");
    println!("{}",name);
}

```

## Other String functions

- to_string
- push_str
- push

```rust
fn main(){
    
    let number = 2020;
    let number_as_string= number.to_string();
    println!("{}",number_as_string);
    println!("{}",number_as_string=="2020");
}

```

Example of push and push_str is given below

```rust
  fn main(){
    
 let mut company = "Tutorial".to_string();
 company.push('s');
 println!("{}",company);
 
 company.push_str(" Point");
  println!("{}",company);
    
}

```

## Concatenation with plus + Operator

+ operator calls the add method of the Add trait. Syntax of add trait is given below.First parameter is a self and second parameter is a reference.
  ```rust
     add(self,&str)->String{

     }
  ```
  
  Let us see string concatenation example.
  
  ```rust
  fn main(){
  let n1 = "Tutorials".to_string();
  let n2 = "Point".to_string();
  
  let n3 = n1 + &n2; // n1 will be moved
  
  println!("{}",n3);
  
   // println!("{}",n1);//Error here
  
  }
  ```

Note that n1 will be moved to the method `add` which is internally called so the line `println!("{}",n1)` will give error as n1 is moved.

## Format! Macro

To solve the problem of concatenating string without ownership change , we can use format macro. It is easy to use format macro than using + operator.

```rust

  fn main(){
  let n1 = "Tutorials".to_string();
  let n2 = "Point".to_string();
  
  let n3 = format!("{} {}",n1,n2);
  
  println!("{}",n3);
  
   // println!("{}",n1);//Error here
  
  }

 ```


 ## How to access characters of String

 You can access string charactes from a string object using string slice.
 
 ```rust
  fn main(){
  let n1 = "Tutorials".to_string();
  
  let c1 = &n1[0..5]; // characters from  0 1 2 3 4
  println!("{}",c1);
  
  }


 ```

The elegant way to access the characters from a string is using `chars` method.Let us see an example.

```rust
 fn main(){
  let n1 = "Tutorials".to_string();
  
  let z = String::new();
  for n in n1.chars(){
      println!("{}",n);
  }
  
  }

```

chars() also work on string literals.

```rust
 fn main(){
  for n in "CodingGround".chars(){
      println!("{}",n);
  }
}

```

<!-- 
1. string functions:
https://doc.rust-lang.org/std/string/struct.String.html

2. string slice:
https://doc.rust-lang.org/std/primitive.str.html#method.chars

3.string literal primitive type
https://doc.rust-lang.org/std/str/index.html


-->