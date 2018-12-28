# Collections

<!--https://doc.rust-lang.org/std/collections/index.html -->
Rust's standard collection library provides efficient implementations of the most common general purpose programming data structures

 - Vector
 - HashMap
 - HashSet

## Vector

A contiguous growable array type, written Vec<T> but pronounced 'vector'.

|Sr No | method |  signature    |Description|
|:----:|:-----|:----------|:-------|
| 1   |new() | pub fn new()->Vect<T>     | Constructs a new, empty Vec<T>.The vector will not allocate until elements are pushed onto it.
|2|push()|pub fn push(&mut self, value: T)|Appends an element to the back of a collection.
|3|remove()|pub fn remove(&mut self, index: usize) -> T|Removes and returns the element at position index within the vector, shifting all elements after it to the left.
|4|contains()|pub fn contains(&self, x: &T) -> bool|Returns true if the slice contains an element with the given value.
|5|len()|pub fn len(&self) -> usize|Returns the number of elements in the vector, also referred to as its 'length'.

 ### Syntax 1 :

 To create a vector we use the static method `new`

```rust
fn main() {
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);
    
    println!("size of vector is :{}",v.len());
    println!("{:?}",v);
}
```
output: 
```rust
size of vector is :3
[20, 30, 40]

```

### Syntax 2:

A simple syntax is using Macro .The type is inferred from value we assign to right.

```rust
fn main() {
    
    let  v = vec![1,2,3];
    println!("{:?}",v);
}
```

output: `[1, 2, 3]`

Following code is not valid as we cannot add different types to vector.

```rust
fn main() {
    let  v = vec![1,2,3,"hello"];
    println!("{:?}",v);
}
```

### Removing element of vector

```rust
fn main() {
    
    let mut v = vec![10,20,30];
    v.remove(1);
    println!("{:?}",v);
}

```
output: `[10, 30]`

### Checking value is present

```rust
fn main() {
    let  v = vec![10,20,30];
    if v.contains(&10){
        println!("found 10");
    }
    println!("{:?}",v);
}

```

output:

```rust
found 10
[10, 20, 30]
```

### Accessing values from a Vector

```rust
fn main() {
    
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    
    
    println!("{:?}",v[0]);
}

ouput is : `20`

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

if we use `for i in v` this program will move the scope to v with in the for loop. As the for loop ends the values will be moved and we cannot run the last line of code `println!("{:?}",v)` as this will give error

output:

```rust
20
30
40
500
[20, 30, 40, 500]
```

### Dereference operator with Vector

When iterating through the list of values, we can use dereference operator to change values of the mutable vector.

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

output: `[21, 31, 41, 501]`

## HashMaps

<!-- https://www.youtube.com/watch?v=sTK8fagTsMk&index=31&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL -->



A map is a collection of key-value pairs (called entries). No two entries have the same key, and the entries are kept organized so that if you have a key, you can efficiently look up the corresponding value in a map. In short, a map is a lookup table.

A HashMap stores the keys and values in a hash table, so it requires a key type K that implements Hash and Eq, the standard traits for hashing and equality. Looking up a value by its key is fast. The entries are stored in an arbitrary order.



|Sr No | method |  signature    |Description|
|:----:|:-----|:----------|:-------|
| 1   |new() | pub fn new() -> HashMap<K, V, RandomState>    | Empty hashmap is created with a capacity of 0, so it will not allocate until it is first inserted into.
|2|len()|pub fn len(&self) -> usize|Returns the number of elements in the map.
|3|insert()|pub fn insert(&mut self, k: K, v: V) -> Option<V>|Inserts a key/value pair,if no key then None is returned . After update old value is returned.
|4|get()|pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V> where K:Borrow<Q> Q:Hash+ Eq|Returns a reference to the value corresponding to the key.
|5|contains_key|pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool |Returns true if the map contains a value for the specified key.
|6|remove()|pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>|Removes a key from the map, returning the stored key and value if the key was previously in the map.
|7|iter()|pub fn iter(&self) -> Iter<K, V>|An iterator visiting all key-value pairs in arbitrary order. The iterator element type is (&'a K, &'a V).

### Illustration:new and insert

```rust

 use std::collections::HashMap;
 fn main(){
    let mut stateCodes = HashMap::new();
    stateCodes.insert("KL","Kerala");
    stateCodes.insert("MH","Maharashtra");

   println!("size of map is {}",stateCodes.len());
   println!("{:?}",stateCodes);
}

```rust
output: ```rust

size of map is 2
{"KL": "Kerala", "MH": "Maharashtra"}

```

### Illustration: get

In this example we are trying to retrieve the value for key *KL* in the map.

```rust

use std::collections::HashMap;
fn main(){
    let mut stateCodes = HashMap::new();
    stateCodes.insert("KL","Kerala");
    stateCodes.insert("MH","Maharashtra");

   println!("size of map is {}",stateCodes.len());
  println!("{:?}",stateCodes);
  
    match stateCodes.get(&"KL") {
        Some(value)=>{
            println!("Value for key KL is {}",value);
        }
        None =>{
            println!("nothing found");
        }
    }
}

  ```

output

```rust
size of map is 2
{"KL": "Kerala", "MH": "Maharashtra"}
Value for key KL is Kerala

```

### Illustration: iterator

  ```rust
   use std::collections::HashMap;
   fn main(){
    let mut stateCodes = HashMap::new();
    stateCodes.insert("KL","Kerala");
    stateCodes.insert("MH","Maharashtra");

    for (key, val) in stateCodes.iter() {
    println!("key: {} val: {}", key, val);
   }
}

  ```

output:

```rust
key: MH val: Maharashtra
key: KL val: Kerala
```

### Illustraion: remove and contains_key

```rust
 use std::collections::HashMap;
fn main(){
    let mut stateCodes = HashMap::new();
    stateCodes.insert("KL","Kerala");
    stateCodes.insert("MH","Maharashtra");
    stateCodes.insert("GJ","Gujarat");
  
    if stateCodes.contains_key(&"GJ"){
     println!("found key");
     stateCodes.remove(&"GJ");
    }

    for (key, val) in stateCodes.iter() {
    println!("key: {} val: {}", key, val);
  }
}


```

output:

```rust
found key
key: MH val: Maharashtra
key: KL val: Kerala

```

### Illustration: or_insert

In Hashmap if the same key is inserted twice the last value will be updated.To avoid this use `or_insert`

```rust

use std::collections::HashMap;
fn main() {

 let mut locations = HashMap::new();
 locations.insert("Mumbai",101);
 locations.entry("Mumbai").or_insert(201);
 locations.entry("Pune").or_insert(301);
 println!("{:?}",locations);

```

output : `{"Pune": 301, "Mumbai": 101}`

## HashSet

HashSet<T> is a set of unique values of type T. Adding and removing values is fast, and it’s fast to ask whether a given value is in the set or not.

Sr No | method |  signature    |Description|
|:----:|:-----|:----------|:-------|
|1|new()|pub fn new() -> HashSet<T, RandomState>|Creates an empty HashSet.The hash set is initially created with a capacity of 0, so it will not allocate until it is first inserted into
|2|len()|pub fn len(&self) -> usize|Returns the number of elements in the set.
|3|insert()|pub fn insert(&mut self, value: T) -> bool|Adds a value to the set.If the set did not have this value present, true is returned else false
|4|get() |pub fn get<Q:?Sized>(&self, value: &Q) -> Option<&T> where T: Borrow<Q>,Q: Hash + Eq,|Returns a reference to the value in the set, if any, that is equal to the given value.
|5|remove()|pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool |Removes a value from the set. Returns true if the value was present in the set.
|6|contains()|pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool |Returns true if the set contains a value.

### Illustraion:new and length

Note from example duplicates values are not added.

```rust
use std::collections::HashSet;

fn main() {

let mut names = HashSet::new();

names.insert("Mohtashim");
names.insert("Kannan");
names.insert("TutorialsPoint");
names.insert("Mohtashim");//duplicates not added

println!("size of the set is {}",names.len());
println!("{:?}",names);



}


```

output is :

```rust
size of the set is 3
{"Kannan", "TutorialsPoint", "Mohtashim"}

```

### Illustration: Displaying values


```rust
use std::collections::HashSet;

fn main() {

let mut names = HashSet::new();

names.insert("Mohtashim");
names.insert("Kannan");
names.insert("TutorialsPoint");
names.insert("Mohtashim");

for name in names.iter(){
    println!("{}",name);
}



}

```

ouput:

```rust
TutorialsPoint
Mohtashim
Kannan

```

### Illustration:get


```rust
use std::collections::HashSet;

fn main() {

let mut names = HashSet::new();

names.insert("Mohtashim");
names.insert("Kannan");
names.insert("TutorialsPoint");
names.insert("Mohtashim");


match names.get(&"Mohtashim"){
    
    Some(value)=>{
        println!("found {}",value);
    }
    None =>{
        println!("not found");
    }
}

println!("{:?}",names);

}

```

output:

```rust
found Mohtashim
{"Kannan", "Mohtashim", "TutorialsPoint"}
```


### Illustraion:contains() and remove()

```rust

use std::collections::HashSet;

fn main() {

let mut names = HashSet::new();

names.insert("Mohtashim");
names.insert("Kannan");
names.insert("TutorialsPoint");



if names.contains(&"Kannan"){
 println!("found name to remove");
  names.remove(&"Kannan");    
}

println!("{:?}",names);

}
```

output: 

```rust
found name to remove
{"TutorialsPoint", "Mohtashim"}

```