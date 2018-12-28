# Structure

Arrays are used to represent a homogeneous collection of values. Similarly ,**structure** is another user defined data type available in Rust that allows us to combine data items of different types,including another structure. A structure defines data as a key-value pair.

### Syntax : Declaring a structure

The `struct` keyword is used to declare a structure. Since structures are statically typed,every field in the structure must be associated with a data type. The naming rules and conventions for a structure is similar to that of a variable. The structure block must end with a semicolon.

```rust
 struct Name_of_structure {
     field1:data_type,
     field2:data_type,
     field3:data_type
 };
```

### Syntax: Initializing a structure

After declaring a struct, each field should be assigned a value. This is known as initialization. 

```rust
let instance_name =Name_of_structure {
    field1:value1,
    field2:value2,
    field3:value3
}; //Note the semicolon

```

### Syntax : Accessing values in a structure

Use the dot notation to access a specific field's value.

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

The above example declares a struct Employee with three fields-name,company and age of types. The main() initializes the structure. It uses the println() to print values of the fields defined in the structure.

Output:

`Name is :Mohtashim company is TutorialsPoint age is 50`

### Modifying a struct instance

To modify an instance, the instance variable should be marked mutable. The below example declares and initializes a structure named `Employee` and later modifies value of the `age` field to 40 from 50. 

```rust
 let mut emp1 = Employee{
      company:String::from("TutorialsPoint"),
       name:String::from("Mohtashim"),
       age:50
  };
  emp1.age=40;
  println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);
  
```
Output:

`Name is :Mohtashim company is TutorialsPoint age is 40`

## Passing a struct to a function

The following example shows how to pass instance of struct as a parameter

The display method takes an Employee instance as parameter and prints the details.

```rust
fn display( emp:Employee){

 println!("Name is :{} company is {} age is {}",emp.name,emp.company,emp.age);
}

```

The complete program and its output is shown as below.

```rust
//declare a structure
 struct Employee{       
    name:String,
    company:String,
    age:u32
}


  fn main() {
  
  //initialize a structure
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
  
  //pass emp1 and emp2 to display()
 display(emp1);
 display(emp2);
  
  
}

//fetch values of specific structure fields using the . operator and print it to the console

fn display( emp:Employee){
 println!("Name is :{} company is {} age is {}",emp.name,emp.company,emp.age);
}


```

Output

```rust
Name is :Mohtashim company is TutorialsPoint age is 50
Name is :Kannan company is TutorialsPoint age is 32

```

## Returning Struct from a function

 Lets consider a function `who_is_elder` which compares two employees age and returns the elder one.

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
  
  //initialize structure 
  
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
 
 //prints details of the elder employee
 display(elder);
  
}

//accepts instances of employee structure and compares their age

fn who_is_elder (emp1:Employee,emp2:Employee)->Employee{
    if emp1.age>emp2.age {
        return emp1;
    }
    else {
    return emp2;
    }
}

//display name, comapny and age of the employee
fn display( emp:Employee){
 println!("Name is :{} company is {} age is {}",emp.name,emp.company,emp.age);
}

//declare a structure
struct Employee{
    name:String,
    company:String,
    age:u32
}

```

Output

```rust

elder is:
Name is :Mohtashim company is TutorialsPoint age is 50

```

## Method in structure

Methods are similar to functions. They are a logical group of programming instructions. Methods are declared with `fn` keyword.The scope of a method is within the structure block.

Methods are declared outside the structure block. The `impl` keyword is used to define a method within the context of a structure. The first parameter of a method will be always `self`,which represents the calling instance of the structure. Methods operate on the data members of a structure.

To invoke a method we need to first instantiate the structure. The method can be called using the structure's instance.

### Syntax

```rust
 struct My_struct {}

 impl My_struct{     //set the method's context
     fn method_name(){  //define a method

     }
 }

```

### Illustration

The following example defines a structure `Rectangle` with fields `width` and `height`. A method `area` is defined within the structure's context.The `area` method accesses the structure's fields via the `self` keyword and calculates the area of a rectangle. 

```rust

//define dimensions of a rectangle
struct Rectangle{
    width:u32,
    height:u32
}

//logic to calculate area of a rectangle

impl Rectangle{
    fn area(&self)->u32 {  //use the . operator to fetch the value of a field via the self keyword
        self.width * self.height
    }
}

fn main(){

//instanatiate the structure
    let small = Rectangle{
        width:10,
        height:20
    };

//print the rectangle's area 
    println!("width is {} height is {} area of Rectangle is {}",small.width,small.height,small.area());
}

```

Output

`width is 10 height is 20 area of Rectangle is 200`


## Static Method in structure

Static methods can be used as utility methods. These methods exist even before the structure is instantiated. Static methods are invoked using the structure's name and can be accessed without an instance.Unlike normal methods, a static method will not take the &self parameter.

### Syntax: Declaring a static method

A static method like functions and other methods can optionally contain parameters.

```rust

impl Structure_Name {

//static method that creates objects of the Point structure
    fn method_name(param1: datatype, param2: datatype) -> return_type {

   // logic goes here
}

```

### Syntax: Invoking a static method

The `structure_name ::` syntax is used to access a static method.

```rust

structure_name::method_name(v1,v2)

```

### Illustration

The following example uses the `getInstance` method as a factory class that creates and returns instances of the structure `Point`.

```rust
//declare a structure
struct Point {
    x: i32,
    y: i32,
}

impl Point {

//static method that creates objects of the Point structure
    fn getInstance(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

//display values of the structure's field
    fn display(&self){
        println!("x ={} y={}",self.x,self.y );
    }
}


fn main(){

// Invoke the static method
    let p1 = Point::getInstance(10,20);
    p1.display();

}

```

Output

`x =10 y=20`

<!-- 
## Struct Update Syntax
## Method in structure
- Methods are functions inside a structure(similar to class in other OOP languages) or enum

method will take self as argument as given below
-->
