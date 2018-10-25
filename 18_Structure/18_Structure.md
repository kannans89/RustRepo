# Structure

Struts are similar to tuples.That means a struct can have data of different types.Each field will
have a meaningful name in structure . Struts doesn't have an order to get or set values. The structure itself will be given a meaningful name.This makes it very flexible over a tuple.

## Syntax

```rust
 struct Name_of_structure {
     field1:data_type,
     field2:data_type,
     field3:data_type
 }
```

After declaring a struct we need to instantiate it,that is we need to provide concete values to each field.Following shows the syntax 

```rust
let instance_name =Name_of_structure {
    field1:value1,
    field2:value2,
    field3:value3
}; //Note the semicolon

```

## Example

```rust
fn main() {
  
  let emp1 = Employee{
     
      company:String::from("TutorialsPoint"),
       name:String::from("Mohtashim"),
       age:50
  };
  
  println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);
  
  
}

struct Employee{
    name:String,
    company:String,
    age:u32
}

```

<!-- TODO: priting struct directly gives error #[Derive(Debug)] -->