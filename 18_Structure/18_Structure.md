# Structure

Arrays allow to define type of variables that can hold several data items of the same kind. Similarly **structure** is another user defined data type available in C that allows to combine data items of different kinds.

### Syntax : Declaring s structure

```rust
 struct Name_of_structure {
     field1:data_type,
     field2:data_type,
     field3:data_type
 }
```

After declaring a struct we need to instantiate it,that is we need to provide concrete values to each field.

### Syntax: Initializing a structure

```rust
let instance_name =Name_of_structure {
    field1:value1,
    field2:value2,
    field3:value3
}; //Note the semicolon

```

### Syntax : Accessing values in a structure

Use the dot notation to access a specific field's value 

`instance_name.field1`


## Illustration

```rust

struct Employee{
    name:String,
    company:String,
    age:u32
}

fn main() {
  
  let emp1 = Employee{
      company:String::from("TutorialsPoint"),
       name:String::from("Mohtashim"),
       age:50
  };
  
  println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);
  
}


```

The above example declares a struct Employee with three fields-name,company and age. The main() initializes the structure. It uses the println() to print values of the fields defined in the structure.

output:

`Name is :Mohtashim company is TutorialsPoint age is 50`

### Modifying a struct instance

To modify an instance, the instance variable should be marked mutable. The below example declares and initializes a structure named `Employee` and later modify value of the `age` field to 40 from 50. 

```rust
 let mut emp1 = Employee{
      company:String::from("TutorialsPoint"),
       name:String::from("Mohtashim"),
       age:50
  };
  emp1.age=40;
  println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);
  
```

`Name is :Mohtashim company is TutorialsPoint age is 40`

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
 struct Employee{
    name:String,
    company:String,
    age:u32
}


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


```

output will be as shown

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

## Method in structure

Methods are similar to function as it is declared with `fn` keyword.Methods are defined within the construct of a struct.
To use a method we need to first instantiated the struct and through
the instance we can call the method.The first parameter of method will be always self,which represents the calling instance of structure.

### Syntax

```rust
 struct My_struct {}

 impl My_struct{
     fn method_name(){

     }
 }

```

To undserstand methods we are creating a structure `Rectangle` with method `area` which will calculate the area of Rectangle. Following is code.

```rust
struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle{
    fn area(&self)->u32 {
        self.width * self.height
    }
}

fn main(){
    let small = Rectangle{
        width:10,
        height:20
    };

    println!("width is {} height is {} area of Rectangle is {}",small.width,small.height,small.area());
}

```

output `width is 10 height is 20 area of Rectangle is 200`


<!-- 
## Struct Update Syntax
## Method in structure
- Methods are functions inside a structure(similar to class in other OOP languages) or enum

method will take self as argument as given below
-->
