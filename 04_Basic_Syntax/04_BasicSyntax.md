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

 Open the REPL shell [here](https://www.tutorialspoint.com/compile_rust_online.php) .Execute the program as shown below.


 


<!-- External links for this chapter

1. http://devdocs.io/rust/book/second-edition/ch01-02-hello-world

2. https://app.pluralsight.com/player?course=rust-fundamentals&author=dmitri-nesteruk&name=rust-fundamentals-m1&clip=3&mode=live

-->