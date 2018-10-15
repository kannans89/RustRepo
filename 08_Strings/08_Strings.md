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

```rust
fn main() {
    let mut name=String::from("Tutorials");
    name.push_str(" Point");
    println!("{}",name);
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