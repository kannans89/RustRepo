#Input Output


## CommandLine Arguments

CommandLine arguments are passed to a program before executing it. CommandLine arguments are similar to parameters passed to functions. CommandLIne arguments can be used to pass values before the execution of a program. Commandline parameters can be used to pass values to the main() function.

### Illustration
The following example passes values as commandLine arguments to the main() function. 

```rust

fn main(){
    let cmd_line = std::env::args();
    println!("No of elements in arguments is :{}",cmd_line.len()); //print total number of values passed

    for arg in cmd_line {
        println!("[{}]",arg); //print all values passed as commandline arguments
    }
   
}

```

Run the above program as file name followed by the commandline arguments separated by space. (Here, `main.exe   hello    tutorialspoint` )

Output 

```rust
No of elements in arguments is :3
[main.exe]
[hello]
[tutorialspoint]

```

### Illustration

The following program prints the sum of values passed as commandline arguments.

```rust


fn main(){
    let cmd_line = std::env::args();
    println!("No of elements in arguments is :{}",cmd_line.len());  // total number of elements passed
    
    let mut sum =0;
    let mut has_read_first_arg = false;
    
    //iterate through all the arguments and calculate their sum 
    
    for arg in cmd_line {
       if has_read_first_arg {  //first argument is the exe file name
       sum += arg.parse::<i32>().unwrap();
       }
       has_read_first_arg = true;
    }

   println!("sum is {}",sum);
}

```

On executing the program as  ` main.exe 1 2 3 4` , the output will be -

```rust
No of elements in arguments is :5
sum is 10
```

## Reading from the Console

The program might have to accept values from the user at runtime. The following example reads values from the standard input  (Keyboard) and prints it to the console.

```rust

fn main(){
    let mut line = String::new();
    println!("Enter your name :");
    std::io::stdin().read_line(&mut line);
    println!("Hello , {}", line);
}

```

The "stdin" function returns a handle to the standard input stream of the current process, to which the "read_line" function can be applied. It waits for an end-of-line character from the standard input stream and tries to read all the characters present in the input buffer.

Output

```rust
Enter your name :
Mohtashim
Hello , Mohtashim

```


## Writing to Console

We have used the  "print" or "println" macros to display text on the console. However, you can also use  the write() standard library function to display some text to the standard output .

Let us see an example.

```rust
    use std::io::Write;

fn main(){

        std::io::stdout().write("Tutorials ".as_bytes()).unwrap();
        std::io::stdout().write(String::from("Point").as_bytes()).unwrap();
}
```

The "stdout" standard library function returns a handle to the standard output stream of the current process, to which the "write" function can be applied.

The "write" function gets an argument of "&[u8]" type, which is a reference to a slice of bytes. Such bytes are printed to the console as an UTF-8 string. So, if you want to print an object that is not a slice of bytes in UTF-8 format, use the "as_bytes" function. 

//explanation for wrap() function goes here 
<!-- .unwrap() function significance , what is fallible?? -->
