# String

String type in Rust can be classifed as-

1. String Literal (**&str**)
2. String Object(**String**)

## String Literal

String literals(&str) are used when value of the string is known at compile time .String literals are a set of characters which are hardcoded into a variable. For example `let company="TutorialsPoint"` . String literals are found in module *std::str*. String literals are also knwn as string slices.

The following example declares two string literals-comapny and location.

```rust
 fn main() {
    let company:&str="TutorialsPoint";
    let location:&str = "Hyderabad";
    println!("company is : {} location :{}",company,location);
}

```

String literals are static by default. This means that string literals are guaranteed to be valid for the duration of the entire program.We can also explicitly specify the variable as static as shown below-
```rust
  fn main() {
 let company:&'static str="TutorialsPoint";
 let location:&'static str = "Hyderabad";
 println!("company is : {} location :{}",company,location);
}
```
The output of the above program will be - 

`company is : TutorialsPoint location :Hyderabad`

## String Object

The String object type is provided in Standard Library. Unlike string literal, the string object type is not a part of the core language.
//need clarity 
It is programmed as a public structure as shown `pub struct String`.Unlike string literal **String** uses ownership concept of Rust language. 

String is a growable collection.It is mutable and UTF-8 encoded type.

The **String**  object type can be used to represent string values that are provided at runtime.

 String object is allocated in the heap . 
 
 //is this reqd 
 When you want to grow your string dynamically like a vector we need to use String object.

### Syntax

 To create a String object you use any of the following syntax
 1. String::new()
   This creates an empty string
 2. String::from()
    This creates a string with some default value passed as parameter to the from() method.

The following example illustrates the use of a String object.

```rust
fn main(){
    let empty_string = String::new();
    println!("length is {}",empty_string.len());

    let content_string = String::from("TutorialsPoint");
    println!("length is {}",content_string.len());
}

```
//explain code here 

The output is as shown below

```rust
length is 0
length is 14

```

### Common methods - String Object

|Sr No |  method | signature  | description
|:----:|:----------|:----|:-----------------|
| 1    | new()     | pub const fn new() -> String|Creates a new empty String.
| 2    | as_str()  | pub fn as_str(&self) -> &str  | Extracts a string slice containing the entire string.
| 3    | push()     |pub fn push(&mut self, ch: char) |Appends the given char to the end of this String.
| 4    | push_str() |pub fn push_str(&mut self, string: &str)   | Appends a given string slice onto the end of this String.
| 5    | len()     |pub fn len(&self) -> usize |Returns the length of this String, in bytes.
| 6   | chars()     |pub fn chars(&self) -> Chars |Returns an iterator over the chars of a string slice.
| 7   | is_empty()  |pub fn is_empty(&self) -> bool |Returns true if input string is empty.
| 8   | split_whitespace()  |pub fn split_whitespace(&self) -> SplitWhitespace |Split a string slice by whitespace,return an iterator
| 9  | contains()  |pub fn contains<'a, P>(&'a self, pat: P) -> bool  |Returns true if the given pattern matches a sub string of input string.
|10|replace()|pub fn replace<'a, P>(&'a self, from: P, to: &str) -> String |Replaces all matches of a pattern with another string.
|11|trim()|pub fn trim(&self) -> &str |Returns a string slice with leading and trailing whitespace removed
|12|split()|pub fn split<'a, P>(&'a self, pat: P) -> Split<'a, P> , where P is pattern  can be &str, char, or a closure that determines the split. |Return an iterator over substrings of this string slice, separated by characters matched by a pattern.

## Convert a String literal to Object

   To access all methods of String object we can easily convert a string literal to object type using `to_string()` method.
   Let us see an example.

```rust
fn main(){

    let name1 = "Hello TutorialsPoint".to_string(); //String object
    let name2 = name1.replace("Hello","Howdy");

    println!("{}",name2);

}
```
//explain code here 

The output of the above code will be   `Howdy TutorialsPoint`

We can also use String::from() method to convert a string literal to string object type as shown. We are appending another string to original string using push_str method.

```rust
fn main() {
    let mut name=String::from("Tutorials");
    name.push_str(" Point");
    println!("{}",name);
}

```

The output will be 

`Tutorials Point`


## Illustration :Type Casting- Number to String  

The following example ilustrates converting a number to a string object

```rust
fn main(){
    let number = 2020;
    let number_as_string= number.to_string(); // convert number to string
    println!("{}",number_as_string);
    println!("{}",number_as_string=="2020");
}

```
The output will be :
//output goes here


## Illustration: push() and push_str() 

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
The output will be :

//paste output here 


## Illustration: len() and trim()

Example to use trim method in string .It removes spaces before and after the content.

```rust
  fn main() {
  
  let fullname = "     Tutorials Point  \r\n";
  println!("Before trim :\n{}",fullname);
   println!("length is  {}",fullname.len());
   
   println!();
   println!("After  trim :\n{}",fullname.trim());
   println!("length is  {}",fullname.trim().len());
}

```

The output will be :

```rust
 Before trim :
     Tutorials Point  

length is  24

After  trim :
Tutorials Point
length is  15

```

## Concatenation of Strings with + operator

A string value can be appended to another string. This is called concatenation or interpolation. The result of string concatenation is a new string object.
The *+* operator internally uses an *add* method . Syntax of this add function takes two parameters.First parameter is *self* i.e. String object itself and the second parameter is a reference of the second string object. This is shown below-

  ```rust
  //add function
     add(self,&str)->String{ // returns a String object

     }

```

## Illustration- String Concatenation.

```rust

  fn main(){
  let n1 = "Tutorials".to_string();
  let n2 = "Point".to_string();
  
  let n3 = n1 + &n2; // n2 reference is passed 
  
  println!("{}",n3);
  
  }
  
```

The Output will be as given below:

`TutorialsPoint`

## Illustration- Format! Macro

Another way to add to String objects together is usign a macro function called format . The use of Format! is as shown below.

```rust

  fn main(){
  let n1 = "Tutorials".to_string();
  let n2 = "Point".to_string();
  
  let n3 = format!("{} {}",n1,n2);
  
  println!("{}",n3);
  
  }

 ```
//output

## Illustration:Split string with white spaces

```rust
   fn main(){
    
    let msg = "Tutorials Point has good tutorials".to_string();
    
    let mut i =1;
    for token in msg.split_whitespace(){
        println!("token {} {}",i,token);
        i+=1;
    }
}

```

Output:

 ```rust
token 1 Tutorials
token 2 Point
token 3 has
token 4 good
token 5 tutorials

```

## Illustration: split string with a delimiter

The split methods returns an iterator , so we are using for each loop to iterate over the result. Sometimes we need to store the split result in a collection , so we can use `collect` method as shown below.Collect method returns a Vector.

```rust
fn main() {
  
  let fullname = "Kannan,Sudhakaran,Tutorialspoint";
  
  for token in  fullname.split(","){
      println!("token is {}",token);
  }
  
  //store in a Vector
  println!("\n");
  let tokens:Vec<&str>= fullname.split(",").collect();
  println!("firstName is {}",tokens[0]);
  println!("lastname is {}",tokens[1]);
  println!("company is {}",tokens[2]);
}

```

output

```rust
token is Kannan
token is Sudhakaran
token is Tutorialspoint

firstName is Kannan
lastname is Sudhakaran
company is Tutorialspoint

```
 ## Accessing chars from string

 You can access string charactes from a string object using string slice function.From string object `Tutorials` we need to slice out `Tutor`.
 Syntax of the string slice function is-
 //add syntax here
 
 The example fetches charcters from index 0 through 4 (5 characters) from the string 'Tutorials'.
 
 ```rust
  fn main(){
  let n1 = "Tutorials".to_string();
  
  let c1 = &n1[0..5]; // characters from  0 1 2 3 4
  println!("{}",c1);
  
  }


 ```
Output : 

`Tutor`

Another way of accessing characters from a string is by using the `chars` method.Let us see an example.

```rust
 fn main(){
  let n1 = "Tutorials".to_string();
  
  let z = String::new();
  for n in n1.chars(){
       println!("{}",n);
  }
  
  }

```
//output


The chars() function can be used  with string literals too. The following example uses the char() function to fetch characters from a string literal.

```rust
 fn main(){
  for n in "CodingGround".chars(){
      println!("{}",n);
  }
}

```

//output

// redo
## String vs &str

`String` object uses dynamic heap to store or modify the string data.`str` is an immutable sequence of UTF-8 bytes of dynamic length somewhere in memory. Since the size is unknown, one can only handle it behind a pointer. This means that str most commonly appears as `&str`

Let us understand the difference with an example as shown . The `print_me(String) ` function takes input as String object.
So if we invoke this function with string literal it should give us error . We will try to convert string object to literal using `as_str()` function and vice versa with `to_string()` method.

```rust
 fn main(){

 print_me(String::from("TutorialsPoint")); // string object

 let s3 = "Hyderabad".to_string(); // convert string literal to object

 let s4= s3.as_str();  // convert object to literal
 
 // print_me(s4);  // Error for literal
 //print_me("Hello I am a literal"); //Error
 
  print_me(s4.to_string());//  literal to object works
  
 }
  
  
  fn print_me(msg: String) {
    println!("the string object is {}", msg);
  }

```

<!-- 
1. string functions:
https://doc.rust-lang.org/std/string/struct.String.html

2. string slice:
https://doc.rust-lang.org/std/primitive.str.html#method.chars

3.string literal primitive type
https://doc.rust-lang.org/std/str/index.html


https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str/24159933#24159933

http://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html

-->
