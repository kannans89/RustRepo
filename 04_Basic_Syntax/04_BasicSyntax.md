# HelloWorld Example

This chapter explains the basic syntax of Rust lang through a **HelloWorld** example.

- Create a **HelloWorld-App** folder and navigate to that folder on terminal

```javascript
C:\Users\Admin>mkdir HelloWorld-App
C:\Users\Admin>cd HelloWorld-App
C:\Users\Admin\HelloWorld-App>

```

- Create a Rust file.Execute the following command:

```javascript

C:\Users\Admin\HelloWorld-App>notepad Hello.rs

```
Rust program files have an extension *.rs*.
The above command will create an empty file **Hello.rs** and opens it in Notepad . Add the code given below to this file-

```rust
fn main(){
 println!("Rust says Hello to TutorialsPoint !!");
}

```

The above program defines a function main `fn main()` . The `fn` keyword is used to define a function. The `main()` is a predefined function that acts as an entry point to the program.
`println` is a predefined macro in Rust. It is used to print a  string (here *Hello*) to the console.
Macro calls are always marked with an exclamation mark `!`.

- Compile the **Hello.rs** file using **rustc**.

```javascript
C:\Users\Admin\HelloWorld-App>rustc Hello.rs

```

On successful compilation of th program, an executable file (*file_name.exe*) is generated.
To verify if the *.exe* file is generated execute the following command.

```javascript
C:\Users\Admin\HelloWorld-App>dir
//lists the files in folder
Hello.exe
Hello.pdb
Hello.rs
```

- Execute the Hello.exe file and verify the output

```javascript
C:\Users\Admin\HelloWorld-App>Hello.exe
//Output of Hello.exe
Rust says Hello to TutorialsPoint !!

```

### What is a macro ?

Rust provides a powerful macro system that allows meta-programming. As you've seen in above example, macros look like functions, except that their name ends with a bang !, but instead of generating a function call, macros are expanded into source code that gets compiled with the rest of the program . So they give more runtime features to program unlike a function. Macros are an extended version of functions.

### println Syntax

Below code shows different syntax of using println.

```rust
println!(); // prints just a newline
println!("hello ");//prints hello 
println!("format {} arguments", "some"); //prints format some arguments

```

### Comments in Rust
Comments are a way to improve the readability of a program. Comments can be used to include additional information about a program like author of the code, hints about a function/ construct etc. Comments are ignored by the compiler.

Rust supports the following types of comments −

- Single-line comments ( // ) − Any text between a // and the end of a line is treated as a comment

- Multi-line comments (/* */) − These comments may span multiple lines.

Example
```rust
//this is single line comment 
 
/* This is a  
   Multi-line comment 
*/
```

## Execute Online

 Rust programs can be executed online through  TutorialsPoint   [Coding Ground](https://www.tutorialspoint.com/compile_rust_online.php) .Write the *HelloWorld* program in the editor tab and click on execute button to view result.

![01_coding_ground](https://user-images.githubusercontent.com/9062443/48670121-4ac63600-eb38-11e8-9f62-b0fb5de88a84.png)

<!-- External links for this chapter

1. http://devdocs.io/rust/book/second-edition/ch01-02-hello-world

2. https://app.pluralsight.com/player?course=rust-fundamentals&author=dmitri-nesteruk&name=rust-fundamentals-m1&clip=3&mode=live

-->
