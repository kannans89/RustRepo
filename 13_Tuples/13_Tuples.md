# Tuple

Tuple is a compound data type . A scalar type can store only one type of
data . For example in an i32 variable can store only a single integer value.In compound types we can store more than one value at a time and it can be of different types.  

Tuples have a fixed length i.e. once declared they cannot grow or shrink in size. The tuple index starts from **0**

## Syntax

```rust
//Syntax1
let tuple_name:(data_type1,data_type2,data_type3) = (value1,value2,value3);

//Syntax2
 let tuple_name = (value1,value2,value3);
```

## Illustration

The following example displays the values in a tuple.

```rust
fn main() {
    let tuple:(i32,f64,u8) = (-325,4.9,22);
    println!("{:?}",tuple);
}

```

The `println!("{}",tuple)` syntax cannot be used to display values in a tuple. This is because a tuple is a compound type .Use the `println!("{:?}",tuple)` syntax to print values in a tuple.

Output

`(-325, 4.9, 22)`

## Illustration 

The following example prints individual values in a tuple.

```rust
fn main() {
    let tuple:(i32,f64,u8) = (-325,4.9,22);
    println!("integer is :{:?}",tuple.0);
    println!("float is :{:?}",tuple.1);
    println!("unsigned integer is :{:?}",tuple.2);
}

```

Output

```rust
integer is :-325
float is :4.9
unsigned integer is :2
```

## Illustration

The following example passes a tuple as parameter to a function. Tuples are passed as values to functions.

```rust
fn main(){

    let b:(i32,bool,f64) = (110,true,10.9);
    print(b);
}

//pass the tuple as a parameter

fn print(x:(i32,bool,f64)){
    println!("Inside print method");
    println!("{:?}",x);
}

```

Output

```rust
Inside print method
(110, true, 10.9)
```

## Destructing

Destructing assignment is a feature of rust in which we unpack the values of a tuple.This is achieved by assigning a tuple to distinct variables.

Consider the following example-

```rust
fn main(){

    let b:(i32,bool,f64) = (30,true,7.9);
    print(b);
}

fn print(x:(i32,bool,f64)){
    println!("Inside print method");
    let (age,is_male,cgpa) = x; //assigns a tuple to distinct variables
    println!("Age is {} , isMale? {},cgpa is {}",age,is_male,cgpa);
}
```

Variable x is a tuple which is assigned to let statement . Each variable age,is_male,cgpa will contain the corresponding values in a  tuple.

output

```rust
Inside print method
Age is 30 , isMale? true,cgpa is 7.9
```