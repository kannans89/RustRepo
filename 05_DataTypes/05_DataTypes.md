
# Data Types

Every value in Rust is of a certain data type.
3.5--> float
'c' --> character
true/false -->boolean

Rust is a statically typed language.
The compiler can usually infer what type we want to use based on the value and how to use it.

Rust can automatically infer the data type  1 if you type--> integer.
We need to define a type of variable in cases when many types are possible , for example we converting a String to numeric type.
eg: '101' --> 101
Two types of data types

- Scalar
- Compound

## Declare a variable

You can declare a variable using `let` keyword.

```rust
//https://www.tutorialspoint.com/compile_rust_online.php

fn main() {
 let company_string = "TutorialsPoint"; // string type
 let rating_float=4.5;  // float type
 let is_growing_boolean=true;  // boolean type
 let icon_char='♥'; //unicode character type

 println!("company name  is:{}",company_string);
 println!("company rating on 5 is:{}",rating_float);
 println!("company is growing  :{}",is_growing_boolean);
 println!("company icon  is:{}",icon_char);

}

```

Note above example we have not  specified the type of the variable.Rust will take default type looking at value assigned . We have used different scalar data types in this example

Output is given below . In println function we are using a special syntax using `{}` , which is the *placeholder* , that will be replaced by the variable value passed after comma.

```rust
company name  is:TutorialsPoint
company rating on 5 is:4.5
company is growing  :true
company icon  is:♥
```

## Scalar Types

A scalar type represents a single value.For eg:  10,3.14,'c' .Rust has four primary scalar types.

- integer
- floating-point
- Booleans
- characters

### Integer

An integer is a number without a fractional component.Integer types in Rust is as shown

|Sr No |  Size    | Signed|UnSigned
|:----:|:----------|:-------|:----------|
| 1    | 8 bit     | i8     | u8        |
| 2    | 16 bit     | i16     | u16     |
| 3    | 32 bit     | i32     | u32     |
| 4    | 64 bit     | i64     | u64     |
| 5    | 128 bit    | i128     | u128   |
| 6    | arch     | isize     | usize   |

signed integers can store both negative as well as positive values.Unsigned integers can only store positive values.The last row shows size of integer is `arch` which means datatype size of signed and unsigned will be derived from the *architecture* of the machine.It will be 32 bit for 32bit machine and if system is 64bit signed and unsigned integer size  will be 64bit.The primary situation in which you’d use isize or usize is when indexing some sort of collection.

Lets see an example to use an integer variable using let statement

```rust
  fn main() {
    let result=10;// i32 by default
    let age:u32= 20;
    let sum:i32 = 5-15;
    let mark:isize=10;
    let count:usize=30;
    println!("result value is {}",result);
    println!("sum is {} and age is {}",sum,age);
    println!("mark is {} and count is {}",mark,count);
 }
```

output is as shown below

```rust
  result value is 10
  sum is -10 and age is 20
  mark is 10 and count is 30
```

You could try replacing number `20` with a floating point value and see what error the compiler gives .

#### Integer range

  Each signed variant can store numbers from `-(2^(n-1) to 2^(n-1) -1` , where n is the number of bits that variant uses. For example *i8* can store numbers from `-(2^7) to 2^7 -1` here we replaced *n* with 8.  

  Each unsigned variant can store numbers from `0 to 2^(n-1)`.For example *u8* can store numbers from `0 to 2^7` which is equal to 0 to 255.
  
#### Integer Overflow

In a variable of integer if you try to store value beyond the range overflow can happen.What will happen if we try to store a value 256 on an unsigned u8 variable . The valid range of unsigned u8 variable is 0 to 255.Let us understand this through an example.

```rust
    fn main() {
    let age:u8= 255;
  
  // 0 to 255 only allowed for u8
    let weight:u8=256;//overflow value is 0
    let height:u8=257;//overflow value is 1
    let score:u8=258;//overflow value is 2
  
    println!("age is {} ",age);
    println!("weight is {}",weight);
    println!("height is {}",height);
    println!("score is {}",score);
 }
```

The output will show a warning `warning: literal out of range for u8` for weight,height and score variables.You can understand from out put that overflow values after 255 will start from 0,1,2 etc. The final output without warning is shown below

```rust
age is 255
weight is 0
height is 1
score is 2
```

### Float

Rust has two primitive types **f32** and **f64** for floating-point numbers.
The f32 type is a single-precision float, and f64 has double precision.The default type is f64.Let us understand this with an example.

```rust
   fn main() {
    let result=10.00;//f64 by default
    let interest:f32=8.35;
    let cost:f64=15000.600; //double precision
    println!("result value  is {}",result);
    println!("interest  is {}",interest);
    println!("cost is {}",cost);
 }
```

output is shown below

```rust
    interest  is 8.35
    cost is 15000.6
```

What will happen in rust if we assign and integer type to floating point.Let us see through an example given below.

```rust
   fn main() {
    let interest:f32=8;// integer assigned to float variable
    println!("interest  is {}",interest);
 }

```

Output shows mismatched types error as given below . Automatic type casting is not allowed in Rust.

```rust
   error[E0308]: mismatched types
 --> main.rs:2:22
  |
2 |     let interest:f32=8;
  |                      ^ expected f32, found integral variable
  |
  = note: expected type `f32`
             found type `{integer}`

error: aborting due to previous error(s)

```

#### Number Separator

For easy readability of large numbers we can use a visual  separator `_` underscore to separate digits.That is 50,000 can be written as 50_000 . This can be understood throught an exmaple.

```rust
  fn main() {
 let float_with_separator=11_000.555_001;
 println!("float value {}",float_with_separator);

 let int_with_separator = 50_000;
 println!("int value {}",int_with_separator);
  }

```

output is :

```rust
float value 11000.555001
int value 50000
```

### Boolean

Boolean types have two possible values true or false.To declare a variable of boolean we use the keyword **bool**

```rust
fn main() {
    let isfun:bool = true;
    println!("Is Rust Programming Fun ? {}",isfun);
 }

```

output will be as shown

`Is Rust Programming Fun ? true`

### Character

Rust supports letters or alphabetic types .To declare a variable of character type we use the keyword **char** . Rust’s char type represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.Let us see an example.

```rust
 fn main() {
 let special_character = '@'; //default
 let alphabet:char = 'A';
 let emoji:char = '😁';
 println!("special character is {}",special_character);
 println!("alphabet is {}",alphabet);
 println!("emoji is {}",emoji);
}
```

 <!--  External  Ref
  The character type :https://www.safaribooksonline.com/library/view/the-rust-programming/9781492067665/xhtml/ch03.xhtml

 --->