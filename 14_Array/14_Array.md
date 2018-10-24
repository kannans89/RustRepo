# Array

Array is a collection of values like tuple. In array all the values is of same type .In an array length is fixed.For variable length data structure we need to use collection.

## Syntax

```rust
 //Syntax1
 let variable_name = [value1,value2,value3];

//Syntax2
let variable_name:[dataType;size] = [value1,value2,value3];

 //Syntax3
let variable_name:[dataType;size] = [default_value_for_elements,size];

```

The array index starts with **0** poistion ,similar to other programming languages.

## Example of an array initialization without any type

Here we are declaring an array of 4 elements and displaying the
entire array using `{:?}` syntax of *println!()* function.The datatype is not specified during the variable declaration.The size of array is computed using `len()` method.

```rust
fn main(){
    let arr = [10,20,30,40];
    println!("array is {:?}",arr);
    println!("array size is :{}",arr.len());
}

```

output is as shown below.

```rust
    array is [10, 20, 30, 40]
    array size is :4

```

## Example of an array initialization with type

In this example we are explicitly specifying the size of the array and the data type the array should be storing as below.

```rust
fn main(){
    let arr:[i32;4] = [10,20,30,40];
    println!("array is {:?}",arr);
    println!("array size is :{}",arr.len());
}

```

output is shown below

```rust
    array is [10, 20, 30, 40]
    array size is :4

```

## Example of initializing array with default values

In this example we are initializing all the elements of the array with default value of `-1`.

```rust

fn main(){
    let arr:[i32;4] = [-1;4];
    println!("array is {:?}",arr);
    println!("array size is :{}",arr.len());
}

```

output is as shown

```rust
 array is [-1, -1, -1, -1]
 array size is :4

```

## Example of iterating array with index

In this example we are iterating across an array and displaying its index
and value present in each index.Note that loop syntax is `0..4` which is 0 to size of the array.

```rust
fn main(){
   let arr:[i32;4] = [10,20,30,40];
    println!("array is {:?}",arr);
    println!("array size is :{}",arr.len());

    for index in 0..4 {
        println!("index is: {} & value is : {}",index,arr[index]);
    }
}
```

output is shown below

```rust
array is [10, 20, 30, 40]
array size is :4
index is: 0 & value is : 10
index is: 1 & value is : 20
index is: 2 & value is : 30
index is: 3 & value is : 40
```

## Example of using iter() method in array

The iterate method returns each element from the array.

```rust

  fn main(){

   let arr:[i32;4] = [10,20,30,40];
    println!("array is {:?}",arr);
    println!("array size is :{}",arr.len());

     for val in arr.iter(){
        println!("value is :{}",val);
    }
  }
```

output is below

```rust
array is [10, 20, 30, 40]
array size is :4
value is :10
value is :20
value is :30
value is :40

```

## Example of mutating the elements of an array

In this example we are making a mutable array and updating the value
of index 1 with 0;

```rust
fn main(){
   let mut arr:[i32;4] = [10,20,30,40];
    arr[1]=0;
    println!("{:?}",arr);
}

```
output is shown below

`[10, 0, 30, 40]`

<!-- TODO: array functions -->