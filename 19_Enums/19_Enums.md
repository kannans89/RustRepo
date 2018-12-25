# Enums

In Rust programming when we have to select a value from a list of possible variants we  use enumeration data types.An enumerated type is declared using the `enum` keyword. Following is the syntax of enum.

```rust
enum enum_name{
    variant1,
    variant2,
    variant3
}

```

## Illustration

The example declares an enum,`GenderCategory`,which have variants as Male and Female . The `print!` macro displays value of the enum.The compiler will throw an error *the trait `std::fmt::Debug` is not implemented for `GenderCategory`*.The attribute `#[derive(Debug)]` is used to suppress this error.

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

```rust
Male
Female
```

## Struct and Enum

The following example defines a structure Person. The field `gender` is of the type `GenderCategory` (which is an enum) and can be assigned either `Male` or `Female` as value.

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

The example creates objects `p1` and `p2` of type Person and initializes the attributes, name and gender for each of these objects.

Output

```rust
Person { name: "Mohtashim", gender: Male }
Person { name: "Amy", gender: Female }
```

## Option Enum

Option is a predefined enum in the Rust standard library . This enum has two values `Some(data)` and `None`.  

### Syntax

```rust
  enum Option<T> {
      Some(T),          //used to return a value
      None                 // used to return null, as Rust doesn't support the null keyword
  }
```
, where, the type *T* represents value of any type.

Rust doesn't support the `null` keyword. The value `None`,in the enum`Option`, can be used by a function to return a null value. If there is data to return, the function can return `Some(data)`.

Let us understand this with an example - 

The program defines a function `is_even()`, with a return type Option. The function verifies if the value passed is an even number. If the input is even ,then a value true is returned, else the function returns `None`.

```rust

fn main() {
    let result = is_even(3);
    println!("{:?}",result);
     println!("{:?}",is_even(30));
}

fn is_even(no:i32)->Option<bool>{
    if no%2 == 0 {
       Some(true)
       }
    else{
       None
       }
}

```

Output

```rust
None
Some(true)
```

## Match statement & Enum

The `match` statement can be used to compare values stored in an enum. The following example defines a function,`print_size`, that takes `CarType` enum as parameter. The function compares the parameter values with a pre-defined set of constants and displays the appropriate message.

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

Output

```rust
Large sized Sports Utility  car
Small sized car
medium sized car

```

## Match with Option

The `is_even` function which returns Option type can be implemented with matched statement as shown below

```rust
fn main() {
     match is_even(5){
         Some(data) => {
             if data==true{
                 println!("Even no");
             }
         },
         None => {
             println!("not even");
         }
     }
}

fn is_even(no:i32)->Option<bool>{
    if no%2 == 0 {
       Some(true)
       }
    else{
       None
       }
}

```

Output

`not even`

## Match & Enum with Data

It is possible to add data type to each variant of a enum.In the following example Name and Usr_ID variants of the enum are of String and integer types respectively.Following example shows enumerations with data and use of match statement.

```rust
// The `derive` attribute automatically creates the implementation
// required to make this `enum` printable with `fmt::Debug`.
#[derive(Debug)]
enum GenderCategory {
     Name(String),Usr_ID(i32)
 }

fn main() {
 let p1 = GenderCategory::Name(String::from("Mohtashim"));
 let p2 = GenderCategory::Usr_ID(100);
 println!("{:?}",p1);
 println!("{:?}",p2);

 match p1 {
     GenderCategory::Name(val)=>{
         println!("{}",val);
     }
     GenderCategory::Usr_ID(val)=>{
         println!("{}",val);
     }
 }
 
 
}

```
