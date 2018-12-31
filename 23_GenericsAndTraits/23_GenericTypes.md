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

## Traits 

Traits can be used to implement a standard set of behaviours (methods) across multiple structures.Traits are similar to **interfaces** in Object Oriented Programming.The syntax of trait as shown below:

### Declare a Trait

```rust
  trait some_trait {
      //abstract or method which is empty
      fn method1(&self);
     // this is already implemented , this is free
      fn method2(&self){
          //some contents of method2
      }
  }
 
```

Traits can contain concrete methods(methods with body)or abstract methods(methods without a body). Use a concrete method if the method definition will be shared by all structures implementing the Trait. However, a structure can choose to override a function defined by the trait.Use abstract methods if the method definition varies for the implementing structures.  
 
### Syntax: Implement a Trait
 
 ```rust

    impl some_trait for structure_name {
      // implement method1() there..
      fn method1(&self ){

      }
  }

 ```

The following examples defines a trait `Printable` with a method `print()`,which is implemented by the structure `book`.

```rust

fn main(){

//create an instance of the structure
    let b1 = Book {
        id:1001,
        name:"Rust in Action"
    };

    b1.print();
}

//declare a structure
struct Book {
  name:&'static str,
  id:u32
}

//declare a trait
trait Printable {
    fn print(&self);
}

//implement the trait
impl Printable for Book {
    fn print(&self){
        println!("Printing book with id:{} and name {}",self.id,self.name)
    }
}

```

Output:

`Printing book with id:1001 and name Rust in Action`

## Generic Functions

The example defines a generic function that displays a parameter passed to it . The parameter can be of any type which implements Display trait as println! macro requires values to implement Display trait.

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

Output

```rust
Inside print_pro generic function:
10
Inside print_pro generic function:
20
Inside print_pro generic function:
Hello TutorialsPoint

```
