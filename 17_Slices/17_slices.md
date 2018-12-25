# Slices

You can access string characters from a string object using  slice syntax.A slice is a part of an array. A sliced string is a pointer to the actual string object.So we need to specify the starting and ending index of a String . Index starts from 0 just like arrays.

Syntax of the string slice function is-

```rust
    let sliced_value = &string_object[start_index..end_index]
```

Note that the end_index will not be included in final string .

The  diagram below shows a sample string `Tutorials` which has length 9 and index starting from 0 to 8.From the input string we are  slicing out `rials` .

![string](https://raw.githubusercontent.com/kannans89/RustRepo/master/Images/string_slice.png)

The following code shows how to slice the string.

```rust

fn main(){
  let n1 = "Tutorials".to_string();
  
  println!("length of string is {}",n1.len());
  let c1 = &n1[4..9]; // characters from  4,5,6,7,8 indexes
  println!("{}",c1);
  
  }


 ```

Output :

```rust
length of string is 9
rials
```

The difference between array and slice is the size of a slice is known only at runtime and not at compile time.
In the given example the array has five elements . We are taking a slice of three elements and passing into another function `use_slice`. Effectively from main we pass the reference of part of the array or slice.
So the `user_slice` method is borrowing part of an array for a while.

```rust
  
fn main(){

    let  data = [10,20,30,40,50];
    use_slice(&data[1..4]);//this is effectively borrowing elements for a while
}

fn use_slice(slice:&[i32]){ // is taking a slice or borrowing a part of an array of i32s

  println!("length of slice is {:?}",slice.len());
  println!("{:?}",slice);

}

```

output is :

```rust
length of slice is 3
[20, 30, 40]
length of slice is 3
[20, 30, 40]
```

## Mutable slices

 Let us see an example to understand mutable slice.

 ```rust

fn main(){

    let mut data = [10,20,30,40,50];
    use_slice(&mut data[1..4]);//this is effectively borrowing elements for a while
    println!("{:?}",data);
}

fn use_slice(slice:&mut [i32]){ // is taking a slice or borrowing a part of an array of i32s

  println!("length of slice is {:?}",slice.len());
  println!("{:?}",slice);
  slice[0]=1010;

}

 ```

output:

```rust

length of slice is 3
[20, 30, 40]
[10, 1010, 30, 40, 50]
```

 the above code passes a mutable slice to `use_slice` function , the function mutates the array by changing element of zero index.

<!-- 
## String Slice

When ever we print text using println!() macro we use double quoted text like "hello" . This actually is a bunch of characters which make a string.
If the string value is known at compile time , it is called a string literal or it is also called a string slice.
Both the statements are actually the same

-  let s ="Hello Rust";
-  let s:&'static str ="Hello Rust";

This means s is a reference to static str . &str is a kind of string type in Rust , which we discussed already in String chapter.

So &str is nothing but a slice into a string.It is statically allocated so we use static keyword and unlike other second String type which is dynamically alllocated.

```rust
fn main(){

    let s:&'static str ="Hello Rust"; //string slice
    println!("{}",s);
}
```

Since it is statically allocated you cannot reassign it so the following code will not work

```rust
fn main() {
let s:&'static str ="Hello Rust";
s="hello again"; // error
let f = s[0]; //error
}
```

In the below program we are displaying the string slice in reverse order.
We are also fetching the first character from the string slice. Finally we display the string slice again. Note the string slice cannot be modified . The chars() and rev() methods helps to iterate across the string slice in reverse order.

```rust
 fn main() {
 let s:&'static str ="Hello Rust"; //string slice

    for c in s.chars().rev(){
        println!("{}",c);
    }

    let r =s.chars().nth(0);
    println!("{:?}",r);
      
    println!("{}",s);
 }

```
output is shown below

```rust
t
s
u
R
 
o
l
l
e
H
Some('H')
Hello Rust
```
-->