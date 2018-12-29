# Generic Types
Generics are a facility to write code for multiple contexts with different types. In Rust, generics refer to the parameterization of datatypes and traits.Generics allows to write more concise and clean code by reducing code duplication and providing type-safety.
The concept of Generics an be applied to methods, functions, structures,enumerations,collections and traits.

The `<T>` syntax ,known as the type parameter, is used to declare a generic construct. `T` represents any data-type.

### Illustration : Generic Collection
The following example declares a vector that can store only integers.

```rust
 fn main(){
    let mut vector_integer: Vec<i32> = vec![20,30];
    vector_integer.push(40);
    println!("{:?}",vector_integer);
}
```
Output: 

```rust
[20, 30, 40]
```

Consider the following snippet :

```rust
fn main(){
    let mut vector_integer: Vec<i32> = vec![20,30];
    vector_integer.push(40);
    vector_integer.push("hello");  //error[E0308]: mismatched types
    println!("{:?}",vector_integer);
}
```
From the above example it is clear that a vector of integer type can only store integer values.So if we try to push a string value into the collection, the compiler will return an error.Generics make collections more type safe.

### Illustration : Generic Structure
The <T> type paramter represents some type ,which the compiler will fill in later.
 
```rust
  struct Data<T> {
    value:T,
        }
   
fn main(){
    //generic type of i32
    let t:Data<i32> = Data{value:350};
    println!("value is :{} ",t.value);
     //generic type of String
    let t2:Data<String> = Data{value:"Tom".to_string()};
    println!("value is :{} ",t2.value);
}
 
```
The above example declares a generic structure named `Data`. The `<T>` type indicates some data type. The `main()` function creates two instances,an integer instance and a string instance, of the structure.

Output:

```rust
value is :350 
value is :Tom 
```

## Generic Functions

Generics is a concept in Rust which promotes code reuse . Generic functions reduce code duplication. The type parameter can 

Generics are specified with type parameters such as
 `fn foo<T>(val:T){}`

  
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

`
## Traits

 Define (isA) Relationship between types . Traits specify behavior.Traits allows us to group types based on behavior. For example anything that can be read from such as a file or a network connection has the Read trait.This is similar to **interfaces** in OOP.

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

output is : `Printing book with id:1001 and name Rust in Action`

