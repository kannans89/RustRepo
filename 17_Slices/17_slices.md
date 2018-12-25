# Slices

A slice is a pointer to a block of memory. Slices can be used to access portions of data stored in contiguous memory blocks. It can be used with data structures like arrays, vectors and strings.Slices use index numbers to access portions of data. The size of a slice is determined  at runtime.

Slices are pointers to the actual data. They are passed by reference to functions, which is also known as borrowing.

For example, slices can be used to fetch a portion of a string value. A sliced string is a pointer to the actual string object.So we need to specify the starting and ending index of a String . Index starts from 0 just like arrays.

### Syntax 

```rust
    let sliced_value = &data_structure[start_index..end_index]
```
The minimum index value is 0 and the maximum index value is the size of the data structure.Note that the end_index will not be included in final string . 

The  diagram below shows a sample string `Tutorials`, that has 9 characters. The index of the first character is 0 and that of the last character is 8.


![string](https://raw.githubusercontent.com/kannans89/RustRepo/master/Images/string_slice.png)

The following code fetches 5 characters from the string (starting from index 4).
```rust

fn main(){
  let n1 = "Tutorials".to_string();
  
  println!("length of string is {}",n1.len());
  let c1 = &n1[4..9]; // fetches characters at 4,5,6,7, and 8 indexes
  println!("{}",c1);
  
  }

 ```

Output :

```rust
length of string is 9
rials
```

## Illustration: Slicing an Integer Array

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
The main() declares an array with 5 elements. It invokes the use_slice() function and passess to it a slice of three elements (points to the data array) . Note that slices are passed by reference. The use_slice() function prints the value of the slice and its length.

Output

```rust
length of slice is 3
[20, 30, 40]
```

## Mutable slices

The &mut keyword can be used to mark a slice as mutable.

```rust

fn main(){

    let mut data = [10,20,30,40,50]; 
    use_slice(&mut data[1..4]);// passes references of 20, 30 and 40 
    println!("{:?}",data);
}

fn use_slice(slice:&mut [i32]){ 

  println!("length of slice is {:?}",slice.len());
  println!("{:?}",slice);
  slice[0]=1010; // replaces 20 with 1010

}

 ```

Output:

```rust

length of slice is 3
[20, 30, 40]
[10, 1010, 30, 40, 50]
```

The  above code passes a mutable slice to the `use_slice` function . The function modifies the second element of the original array.

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
