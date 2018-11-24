# Slices

A slice is a part of an array.The difference between array and slice is the
size of a slice is known only at runtime and not at compile time.

In the given example data array has five elements . We are taking a slice of three elements and passing into another function `use_slice`. Effectively from main we pass the reference of part of the array or slice.
So the `user_slice` method is borrowing part of an array for a while.

```rust
  
fn main(){

    let  data = [10,20,30,40,50];
    use_slice(&data[1..4]);//this is effectively borrowing elements for a while
}

fn use_slice(slice:&[i32]){ // is takign a slice or borrowing a part of an array of i32s

  println!("length of slice is {:?}",slice.len());
  println!("{:?}",slice);

}

```

## Mutable slices

 Let us see an example to understand mutable slice.

 ```rust

fn main(){

    let mut data = [10,20,30,40,50];
    use_slice(&mut data[1..4]);//this is effectively borrowing elements for a while
    println!("{:?}",data);
}

fn use_slice(slice:&mut [i32]){ // is takign a slice or borrowing a part of an array of i32s

  println!("length of slice is {:?}",slice.len());
  println!("{:?}",slice);
  slice[0]=1010;

}

 ```

 the following code passes a mutable slice to `use_slice` function , the function mutates the array by changing element of zero index.

## String Slice

 &str or string slice is a fairly inflexible object.