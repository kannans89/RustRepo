# Collections

## Vector

Vector is a collection

```rust
fn main() {
    
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);
    
    println!("{:?}",v);
}

```

Vector using Macro 
The type is infered from value we assign to right.

```rust
fn main() {
    
    let mut v = vec![1,2,3];
    println!("{:?}",v);
}
```

Following code is not valid

```rust
fn main() {
    
    let mut v = vec![1,2,3,"hello"];
    println!("{:?}",v);
}
```

Accessing values from a Vector

```rust
fn main() {
    
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    
    
    println!("{:?}",v[0]);
}

Display values in vector using reference

```rust

 fn main() {
    
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);
    v.push(500);
    
    //vector scope is not moved
   for i in &v {
       println!("{}",i);
   }
   //since scope not moved ,its is accessible here
   println!("{:?}",v);
}

```

if we use `for i in v` this program will move the scope to v with in the for loop. As the for loop ends the values will be moved and we cannot run the  `println!("{:?}",v)` as this will give error

Dereference operator with Vector

```rust
fn main() {
    
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);
    v.push(500);
    
   for i in &mut v {
       *i+=1; // dereference operator
   }
   
   println!("{:?}",v);
}
```

## Storing elements of different types in Vector

example

```rust
#[derive(Debug)]
enum SpreadSheet {
    IntColumn(i32),
    FloatColumn(f64),
    TextColumn(String)
    
}

fn main(){
    let row = vec![SpreadSheet::IntColumn(10),SpreadSheet::FloatColumn(10.55),SpreadSheet::TextColumn(String::from("TutorialsPoint"))];
    
    
    println!("{:?}",row);
    
}

```
