# HelloWorld

- Check the rust compiler path is set properly by opening a terminal and typing

```javascript
C:\Users\Admin>rustc --version
rustc 1.29.0 (aa3ca1994 2018-09-11)
```

- Crate a **HelloWorld-App** folder and navigate to that folder on termainal

```javascript
C:\Users\Admin>mkdir HelloWorld-App
C:\Users\Admin>cd HelloWorld-App
C:\Users\Admin\HelloWorld-App>

```

- Crate a **Hello.rs** file and open in notepad as given below

```javascript

C:\Users\Admin\HelloWorld-App>notepad Hello.rs

```

This will create a file name **Hello.rs** and opens it in notepad . Now lets type the first program as given below inside the file.

```rust
fn main(){
 println!("Rust says Hello to TutorialsPoint !!");
}

```

- Compile the **Hello.rs** file as given below.

```javascript
C:\Users\Admin\HelloWorld-App>rustc Hello.rs

```

Once the compilation is successful it will generate an executable files you can verify if exe file is generated as given below.

```javascript
C:\Users\Admin\HelloWorld-App>dir
//lists the files in folder
Hello.exe
Hello.pdb
Hello.rs
```

- Now lets execute the Hello.exe file and verify the output

```javascript
C:\Users\Admin\HelloWorld-App>Hello.exe

Rust says Hello to TutorialsPoint !!

```

## Using Coding Ground

 Open the REPL shell [here](https://www.tutorialspoint.com/compile_rust_online.php) .Write code in the shell and execute the program as shown below.

![01_coding_ground](https://user-images.githubusercontent.com/9062443/48670121-4ac63600-eb38-11e8-9f62-b0fb5de88a84.png)

## Syntax Understanding

` println!("Rust says Hello to TutorialsPoint !!");`

- print It is the name of a macro defined in the Rust standard library.

- ! It specifies that the preceding name indicates a macro. Without such a symbol, print would instead indicate a function. There is no such function in the Rust standard library, and so you would get a compilation error. A macro is a thing similar to a function - it’s some Rust code to which a name is associated. By using this name, you ask to insert such code in this point.

- Instead of using a single literal string , you can print several of them, even in a single statement. In this way:
`println!("{}, {}!!", "Rust says Hello to ", "TutorialsPoint");`

In this case, the print macro receives three arguments, separated by commas. All three arguments are literal strings. The first string, though, contains two pairs of braces ({}). They are placeholders, indicating the positions in which to insert the other two strings.

## Comments

The following code given will output 34

```rust
// This program
// prints a number.
print!("{}", 34); // thirty-four
/* print!("{}", 20);
*/

```

Single line comments are indicated using `//` and multiline comments using `/* */`

## Debug Print

As we saw, the "print" and "println" macros accept only a string as their first argument, but the possible further arguments can be of various types, including integer numbers, floating-point numbers, and Boolean values. However, if you want to print the contents of an array or of a vector, the following code is not allowed:
print!("{} {}", [1, 2, 3], vec![4, 5]);
because both the array passed as second argument, and the vector passed as third argument have no standard display format, and so two error messages are emitted.
Though, when debugging a program, it is useful to display the contents of such structures, without having to resort to "for" loops. For this purpose, you can write
print!("{:?} {:?}", [1, 2, 3], vec![4, 5]);
that will print "[1, 2, 3] [4, 5]".
By inserting the characters :? enclosed in the braces of a placeholder, you are telling the print macro (and the println macro) to generate a debug format for the corresponding data. So, whenever you want to print the contents of any variable, even if "{}" does not work, you may hope that "{:?}" works.
<!-- External links for this chapter

1. http://devdocs.io/rust/book/second-edition/ch01-02-hello-world

2. https://app.pluralsight.com/player?course=rust-fundamentals&author=dmitri-nesteruk&name=rust-fundamentals-m1&clip=3&mode=live

-->