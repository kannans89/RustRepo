# Structure

Struts are similar to tuples.That means a struct can have data of different types.Each field will have a meaningful name in structure . Struts doesn't have an order to get or set values. The structure itself will be given a meaningful name.This makes it very flexible over a tuple.

## Syntax

```rust
 struct Name_of_structure {
     field1:data_type,
     field2:data_type,
     field3:data_type
 }
```

After declaring a struct we need to instantiate it,that is we need to provide concrete values to each field.Following shows the syntax

```rust
let instance_name =Name_of_structure {
    field1:value1,
    field2:value2,
    field3:value3
}; //Note the semicolon

```

Accessing values using  *dot* notation
To access a specific value of field we can use following syntax `instance_name.field1`

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

### How to modify struct instance created

To modify an instance, the instance variable should be marked mutable for example initial age of **emp1** was 50 and its mutated to 40.

```rust
 let mut emp1 = Employee{
      company:String::from("TutorialsPoint"),
       name:String::from("Mohtashim"),
       age:50
  };
  emp1.age=40;
  println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);
  
```

## Passing Stuct to a function

The following example shows how to pass instance of struct as a parameter
The display method takes an Employee and print the details.

```rust
fn display( emp:Employee){

 println!("Name is :{} company is {} age is {}",emp.name,emp.company,emp.age);
}

```

The full program and output is shown as below.

```rust
  fn main() {
  
  let  emp1 = Employee{
     
      company:String::from("TutorialsPoint"),
       name:String::from("Mohtashim"),
       age:50
  };
 
 let emp2 = Employee{
     
      company:String::from("TutorialsPoint"),
       name:String::from("Kannan"),
       age:32
  };
 

 
 display(emp1);
 display(emp2);
  
  
}


fn display( emp:Employee){
 println!("Name is :{} company is {} age is {}",emp.name,emp.company,emp.age);
     
}

struct Employee{
    name:String,
    company:String,
    age:u32
}

```

output will be as shown

```rust


```rust
Name is :Mohtashim company is TutorialsPoint age is 50
Name is :Kannan company is TutorialsPoint age is 32

```

## Returning Struct from a function

 Lets consider a function `who_is_elder` which compares two employees age and returns the elder.

 ```rust

fn who_is_elder (emp1:Employee,emp2:Employee)->Employee{
    if emp1.age>emp2.age {
        return emp1;
    }
    else {
      return emp2;
    }
}

 ```

The complete program and output is as shown below.

```rust

fn main() {
  
  let  emp1 = Employee{
      company:String::from("TutorialsPoint"),
       name:String::from("Mohtashim"),
       age:50
  };
 
 let emp2 = Employee{
      company:String::from("TutorialsPoint"),
       name:String::from("Kannan"),
       age:32
  };
 
 let elder = who_is_elder(emp1,emp2);
 println!("elder is:");
 display(elder);
  
}

fn who_is_elder (emp1:Employee,emp2:Employee)->Employee{
    if emp1.age>emp2.age {
        return emp1;
    }
    else {
    return emp2;
    }
}

fn display( emp:Employee){
 println!("Name is :{} company is {} age is {}",emp.name,emp.company,emp.age);
}

struct Employee{
    name:String,
    company:String,
    age:u32
}

```

output will be

```rust

elder is:
Name is :Mohtashim company is TutorialsPoint age is 50

```

## Struct Update Syntax


<!-- TODO: priting struct directly gives error #[Derive(Debug)] -->
<!-- TODO:Clone method in Rust -->

## Method in structure

- Methods are functions inside a structure(similar to class in other OOP languages) or enum

method will take self as argument as given below

