# Enums

An enumeration is used to define named constant values. An enumerated type is declared using the enum keyword.

## Syntax

```rust
enum enum_name{
    enum_list
}

```
Where,

- enum_name specifies the enumeration type name
- enum_ist is a comma-separated list of identifiers

### Illustration 

```rust

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

## Enum with Struct


```rust

#[derive(Debug)]
enum GenderCategory {
     Male,Female
 }
 
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
## Putting Data into an enum

Here we are attaching data to each variant of a structure.

```rust

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

## Option Enum

This is another enum defined by the standard library. This is useful in scenarios where value should be something or else nothing.Rust doesn't support **null** values.

### Syntax of Option Enum 

The type *T* is a generic type , which means any type. We discuss generics later in this tutorials.

```rust
  enum Option<T> {
      Some(T),
      None
  }
```

## Example of Opiton

Option enum is already included in the standard library

```rust
  fn main(){
    
    let ip = Some("127.23.81.133");
    let age = Some(18);
    let location:Option<String> = None;


    println!("{:?}",ip);
    println!("{:?}",age);
     println!("{:?}",location);
}

```

output is shwon below

```rust
Some("127.23.81.133")
Some(18)
None

```

## Match with Enum

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

## Using if let with Option Enum

```rust

 fn main(){
    
    let no_10= Some(10);
    
    if let Some(10) = no_10 {
        println!("some enum value is 10");
    }
}

```
