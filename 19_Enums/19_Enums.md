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

The example declares an enum,`GenderCategory`,which have variants as Male and Female . The !print macro displays value of the enum.The compiler will throw an error *the trait `std::fmt::Debug` is not implemented for `GenderCategory`*.The attribute `#[derive(Debug)]` is used to supress this error.

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

ouptut is

```
Male
Female
```

## Struct and Enum

In the example given we are creating a structure Person with gender  type as an enum.

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
The example creates objects `p1` and `p2` of type Person and initializes the attributes name and gender for each of these objects.

Output

```rust
Person { name: "Mohtashim", gender: Male }
Person { name: "Amy", gender: Female }
```

// appu: is this needed?? 
## 

It is possible to add data type to each variant of a enum.In the following example Male and Female variants of the enum are of String type.

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

Option is a predefined enum in the Standard Library .Since Rust doesn't support **null** values  we can return `None` from functions instead of null.If there is data to return from function we can return `Some(data)`. 

### Syntax
The type *T* is a generic type , which means any type. Generics is discussed in detail in a separate chapter.

```rust
  enum Option<T> {
      Some(T),
      None
  }
```

 In the given example functio is_even() is returning an Option<bool> of boolean type.If the even no then Some(true) is retuned otherwise None will be returned.
 
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



## Matching Enum Values

The `match` keyword can be used to compare values stored in an enum. The following example defines a function,`print_size`, that takes `CarType` enum as parameter. The function compares the paramter values with a pre-defined set of constants and displays the appropriate message.

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

The `is_even` function which returns Option type can be matched as shown below

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

output `not even`