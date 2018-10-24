# Tuple

Tuple is a compound data type . In scalar type you can only store one type of
data for example in i32 variable we can store only one single integer value.
In compound types we can store more than one value at a time and it can be of different type.  

Tuples have a fixed length that is once declared they cannot grow or shrink in size. Tuple index starts from **0**

## Syntax

```rust
//Syntax1
let tuple_name:(data_type1,data_type2,data_type3) = (value1,value2,value3);

//Syntax2
 let tuple_name = (value1,value2,value3);
```

## Example to display a tuple

To display a tuple using `println!("{}",tuple)` syntax will not work . This is because tuple is a compound type . We have to use `println!("{:?}",tuple)` syntax.

```rust
fn main() {
    let tuple:(i32,f64,u8) = (-325,4.9,22);
    println!("{:?}",tuple);
}

```

output is `(-325, 4.9, 22)`

## Example accessing tuple index

```rust
fn main() {
    let tuple:(i32,f64,u8) = (-325,4.9,22);
    println!("integer is :{:?}",tuple.0);
    println!("float is :{:?}",tuple.1);
    println!("unsigned integer is :{:?}",tuple.2);
}

```

output is shown below

```rust
integer is :-325
float is :4.9
unsigned integer is :2
```

## Example passing tuple to a function

In the example print function takes a tuple as parameter

```rust
fn main(){

    let b:(i32,bool,f64) = (110,true,10.9);
    print(b);
}

fn print(x:(i32,bool,f64)){
    println!("Inside print method");
    println!("{:?}",x);
}

```

output is shown below

```rust
Inside print method
(110, true, 10.9)
```

## Destructing

When you assign a tuple to a variable it is known as destructing.For example
 in the below example `let (age,is_male,cgpa) = x;` variable x is tuple which is assigned to let statement . Each variable age,is_male,cgpa will contain the respective value of the tuple.

```rust
fn main(){

    let b:(i32,bool,f64) = (30,true,7.9);
    print(b);
}

fn print(x:(i32,bool,f64)){
    println!("Inside print method");
    let (age,is_male,cgpa) = x;
    println!("Age is {} , isMale? {},cgpa is {}",age,is_male,cgpa);
}
```

output is shown below , this shows that we destructed the full tuple into individual variables.

```rust
Inside print method
Age is 30 , isMale? true,cgpa is 7.9
```
