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

## HashMaps

```rust
 
 use std::collections::HashMap;
fn main(){
    let mut stateCodes = HashMap::new();
    stateCodes.insert("KL","Kerala");
    stateCodes.insert("MH","Maharashtra");

    println!("{:?}",stateCodes);
}

```


## Collect

Collect methods helps to convert a vector to a hashmap.

```rust

use std::collections::HashMap;
fn main() {

   let team = vec!["Blue","Red"];
   let score = vec![10,20];
   let scores:HashMap<_,_>=team.iter().zip(score.iter()).collect();
   println!("{:?}",scores);
}

```

Accessing value in a HashMap using get() this will return value if key is found . Otherwise it reutns none.

```rust
use std::collections::HashMap;
fn main() {

 let mut locations = HashMap::new();
 locations.insert("Mumbai",101);
 locations.insert("Pune",102);
 
 let id = locations.get("Mumbai");
 println!("{:?}",id);
  
}

```

output is `Some(101)`


iterate across all key and vlaues


```rust
use std::collections::HashMap;
fn main() {
 let mut locations = HashMap::new();
 locations.insert("Mumbai",101);
 locations.insert("Pune",102);

 for (key,value) in &locations {
     println!("key is :{} , value is :{}",key,value);
 }
}

```

output is as shown

```rust

key is :Mumbai , value is :101
key is :Pune , value is :102

```

## Updating the hashmap

The hashMap is updated if the same key is entered twice . The last value is updated .To avoid this use `or_insert`

```rust

use std::collections::HashMap;
fn main() {

 let mut locations = HashMap::new();
 locations.insert("Mumbai",101);
 locations.entry("Mumbai").or_insert(201);
 locations.entry("Pune").or_insert(301);
 println!("{:?}",locations);

```
