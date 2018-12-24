# Enums
An enumeration defines a set of named integral constants that may be assigned to a variable.An enumerated type is declared using the `enum` keyword. 

 ### Syntax

```rust
enum enum_name{
    variant1,
    variant2,
    variant3
}

```
where,

variant1,variant2 and variant3 represent values in an enum.

## Illustration

The example declares an enum,`GenderCategory`,which have variants as Male and Female . The compiler will throw an error *the trait `std::fmt::Debug` is not implemented for `GenderCategory`* when the value of enum is displayed print! macro is used to display a enum. Use the attribute `#[derive(Debug)]` as shown in the example, to supress this error.

```rust
// The `derive` attribute automatically creates the implementation
// required to make this `enum` printable with `fmt::Debug`.
#[derive(Debug)]
enum GenderCategory {
     Male,Female
 }

fn main() {


let male = GenderCategory::Male;
let female = GenderCategory::Female;


println!("{:?}",male);
println!("{:?}",female);

}
```
Output

```
Male
Female
```

## Struct and Enum

The example given below declares a structure `Person`, with a field `gender` of type enum.

```rust
// The `derive` attribute automatically creates the implementation
// required to make this `enum` printable with `fmt::Debug`.

#[derive(Debug)]
enum GenderCategory {
     Male,Female
 }

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct Person {
    name:String,
    gender:GenderCategory
}

fn main() {


 let p1 = Person{
     name:String::from("Mohtashim"),
     gender:GenderCategory::Male
 };
 
 let p2 = Person{
      name:String::from("Amy"),
     gender:GenderCategory::Female
 };
 
 println!("{:?}",p1);
 println!("{:?}",p2);


}


```

In the example we are creating two objects of Person `p1` and `p2` with specific values to each attributes . After that we are dispalying it into console . The output is -

```rust
Person { name: "Mohtashim", gender: Male }
Person { name: "Amy", gender: Female }
```

## Passing Data to an enum

In Rust it is possible to  add data type to each variant of a enum.In the following example Male and Female variants of the enum are of String type.

```rust
// The `derive` attribute automatically creates the implementation
// required to make this `enum` printable with `fmt::Debug`.
#[derive(Debug)]
enum GenderCategory {
     Male(String),Female(String)
 }
 
fn main() {
 let p1 = GenderCategory::Male(String::from("Mohtashim"));
 let p2 = GenderCategory::Female(String::from("Amy"));
 println!("{:?}",p1);
 println!("{:?}",p2);


}


```

In the example we are passing data as `Mohtashim` and `Amy` while calling Male and Female enums.

## Option Enum

This is another enum defined by the standard library. This is useful in scenarios where value should be something or else nothing.Rust doesn't support **null** values.

Following is the syntax of Option enum.The type *T* is a generic type , which means any type. We will discuss generics in later chapter.

```rust
  enum Option<T> {
      Some(T),
      None
  }
```

Since Option enum is already included in the standard library we do not have to import it to main function.The example shows variables of type `Option` . Since they are of Option type we can only assign either `Some` or `None`

```rust
fn main(){
    let ip:Option<&str> = Some("127.23.81.133");
    let age:Option<i32> = Some(18);
    let location:Option<&str> = None;


    println!("{:?}",ip);
    println!("{:?}",age);
    println!("{:?}",location);
}

```

output is shown below

```rust
Some("127.23.81.133")
Some(18)
None

```

## Matching Enum Values

To compare the values stored in an enum we have to use `match` keyword.In the given example `print_size` function takes CarType enum as a parameter and displays if it is small ,medium or large sized using match statement.

```rust

 enum CarType {
    Hatch,
    Sedan,
    SUV
}

fn print_size(car:CarType){

   match car {
       CarType::Hatch => {
           println!("Small sized car");
       },
       CarType::Sedan => {
           println!("medium sized car");
       },
       CarType::SUV =>{
           println!("Large sized Sports Utility  car");
       }
   }
}


fn main(){
    print_size(CarType::SUV);
     print_size(CarType::Hatch);
      print_size(CarType::Sedan);
}

```

output is :

```rust
Large sized Sports Utility  car
Small sized car
medium sized car

```
