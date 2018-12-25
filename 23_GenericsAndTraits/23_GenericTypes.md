# Generic Types

Removing duplicate by extracting a function .

## Traits

 Define (isA) Relationship between types . Traits specify behavior.
 Traits allows us to group types based on behavior. For example anything that
 can be read from such as a file or a network connection has the Read trait.
 This is similar to interfaces in OOP.

 Traits can have default methods and can also have abstract methods , which need to be implemented. Syntax of trait as shown below

 ```rust
  trait some_trait {
      //abstract or method which is empty
      fn method1(&self);
     // this is already implemented , this is free
      fn method2(&self){
          //some contents of method2
      }
  }
 struct Foo{
     x:u32
 }

  impl some_trait for Foo {
      // implement method1() there..
      fn method1(&self ){

      }
  }

 ```

A good example of trait is Iterator,we implement next() method and will get tons of other methods for free.When a type implements a trait the types get isA relation.Clone and Copy are other popular traits in Rust.Display is another trait.

In the given example we are making Book is a Printable object by implementing print() method.

```rust

fn main(){

    let b1 = Book {
        id:1001,
        name:"Rust in Action"
    };

    b1.print();
}

struct Book {
  name:&'static str,
  id:u32
}

trait Print {
    fn print(&self);
}

impl Print for Book {
    fn print(&self){
        println!("Printing book with id:{} and name {}",self.id,self.name)
    }
}

```

## Generic Functions

Is a powerful concept in rust which promote code reuse . Generic functions reduce code duplication.They take any type that implements some set of traits,and they can only use the behavior of those traits.

Generics are specified with type parameters such as
 `fn foo<T>(val:T){}`
- T is the name of the type,which the compiler will fill in later
  
Let us see an example of  generic function ,
here two different functions are made to display a u8 and u16 values.

```rust
  fn print_u8(val:u8){
    println!("u8 value is :{}",val);
}

fn print_u16(val:u16){
    println!("u16 value is:{}",val);
}

fn main(){
    print_u8(10 as u8);
    print_u16(20 as u16);

```

We can improve this code with help of Display trait as shown below.Here we have a generic function print_pro which can print intergers and string. Since integer and string implemnted Display trait , this is possible

```rust

use std::fmt::Display;

fn main(){
    print_pro(10 as u8);
    print_pro(20 as u16);
    print_pro("Hello TutorialsPoint");
}

fn print_pro<T:Display>(t:T){
    println!("Inside print_pro generic function:");
    println!("{}",t);
}


```

Now let us create a type Book which isA Display trait as shown below.This will allow as to pass Book object directly to print_pro directly.

```rust
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

struct Book{
    id:u32
}
impl Display for Book {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Book id {}", self.id)
    }
}

fn main(){
    print_pro(10);
    print_pro("Hello TutorialsPoint");
    print_pro(Book{id:1001});
}

fn print_pro<T:Display>(t:T){
    println!("Inside print_pro generic function:");
    println!("{}",t);
}

```

## Generic type

Allows to write more concise and clean code.This is useful when creating
collections and other data structures.If your are writing a library this will given more flexibility . For example let us take an example of Vector

```rust
 fn main(){
    let mut vector_integer: Vec<i32> = vec![20,30];
    let mut vector_string_slice:Vec<&'static str> = vec!["Tutorials","Point"];

   vector_string_slice.push("Mohtahsim");
   vector_integer.push(40);
   println!("{:?}",vector_string_slice);
   println!("{:?}",vector_integer);
}
```

Let us create a generic type struct and use it

```rust
  
struct Tagged<T> {
    value:T,
    tag_name:String
}

impl<T> Tagged<T> {
    fn tag(&self)->String {
        self.tag_name.clone()
    }
}

fn main(){
    //generic type of i32
    let t:Tagged<i32> = Tagged{value:350,tag_name:"Kannan".to_string()};
    println!("value is :{} name is:{}",t.value,t.tag());
     //generic type of String
    let t2:Tagged<String> = Tagged{value:"Sudhakaran".to_string(),tag_name:"Kannan".to_string()};
    println!("value is :{} name is:{}",t2.value,t2.tag());
}

```