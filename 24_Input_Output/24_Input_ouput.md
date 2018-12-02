#Input Output


## CommandLine Arguments

CommandLine arguments are passed to a program before executing it . For example before executing a program 'main.exe' we can pass arguments like 'main.exe 10 20 30'.
So 10 20 30 are input to the program main.exe.

```rust

fn main(){
    let cmd_line = std::env::args();
    println!("No of elements in arguments is :{}",cmd_line.len());

    for arg in cmd_line {
        println!("[{}]",arg);
    }
   
}

```

If this program is compiled to create a file named, say, main, and such file is launched writing the command line `main.exe   hello    tutorialspoint`, it will print output as

```rust
No of elements in arguments is :3
[main.exe]
[hello]
[tutorialspoint]

```

The command line parameters include the file name main.exe ,followed by hello and tutorialspoint.

Let us see another complex example where we pass set of nos while executing the program as input and the pgram reads the input values and dispalys output as  shwon

```rust


fn main(){
    let cmd_line = std::env::args();
    println!("No of elements in arguments is :{}",cmd_line.len());
    
    let mut sum =0;
    let mut has_read_first_arg = false;
    for arg in cmd_line {
       if has_read_first_arg {  //first argument is the exe file name
       sum += arg.parse::<i32>().unwrap();
       }
       has_read_first_arg = true;
    }

   println!("sum is {}",sum);
}

```

if file is saved with name main.rs , run it with ` main.exe 1 2 3 4`

```rust
No of elements in arguments is :5
sum is 10
```

## Reading from the Console

For command-line oriented programs, a typical way to get input is to read a line from the keyboard until the user presses Enter.

```rust

fn main(){
    let mut line = String::new();
    println!("Enter your name :");
    std::io::stdin().read_line(&mut line);
    println!("Hello Dear, {}", line);
}

```
output of the program will be as shown

```rust
Enter your name :
Mohtashim
Hello Dear, Mohtashim

```

The "stdin" function returns a handle to the standard input stream of the current process . On that handle, the "read_line" function can be applied. It waits for an end-of-line character from the standard input stream, and then it tries to read all the characters present in the input buffer.

## Writing to Console

We were writing to console from the very first program , but we used "print" or "println" macros, which are implemented using a standard library function. However, you can also directly use library functions to display some text to the console .

Let us see an example.

```rust
    use std::io::Write;

fn main(){

        std::io::stdout().write("Tutorials ".as_bytes()).unwrap();
        std::io::stdout().write(String::from("Point").as_bytes()).unwrap();
}
```

The "stdout" standard library function returns a handle to the standard output stream of the current process. On that handle, the "write" function can be applied.

The "write" function gets an argument of "&[u8]" type, which is a reference to a slice of bytes. Such bytes are printed to the console as an UTF-8 string. So, if you want to print an object that is not a slice of bytes in UTF-8 format, first you have to translate it to such a thing.
To convert both a static string and a dynamic string to a reference to a slice of bytes, you can use the "as_bytes" function. 

<!-- .unwrap() function significance -->