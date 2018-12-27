# File Input/Output

In addition to reading and writing to console,Rust allows reading and writing to files.

The File struct represents a file that has been opened , and gives read or write access to the underlying file.Since many things can go wrong when doing file I/O, all the File methods return the io::Result<T> .


|Sr No |  method    | description|  
|:----:|:----------|:-------|
| 1    |  open()  | The open static method can be used to open a file in read-only mode
| 2    |  create()  | static method opens a file in write-only mode. If the file already existed, the old content is destroyed. Otherwise, a new file is created.

## Write to File

Let us understand this with an example.

The following program creates a file 'data.txt'in the current folder of the file system.The create() method is used to create a file. The method returns a file handle if the file is created successfully.The last line *write_all* function will write bytes in newly created file.

//clarity 
Since this method is fallible we are unwrapping it.

```rust
use std::io::Write;
fn main(){
    let mut file = std::fs::File::create("data.txt").unwrap();
    file.write_all("Hello World".as_bytes()).unwrap();
     file.write_all("\nTutorialsPoint".as_bytes()).unwrap();
}
```

## Read a File

The following program reads the contents in a file *data.txt* and prints it to the console.
The "open" function is used to open an existing file. An absolute or relative path to the file is passed to the open() function as a parameter.
The open() function throws an exception if the file does not exist, or if it is not accessible for whatever reason. If it succeeds, a file handle to such file is assigned to the "file" variable.
//rephrase this 
The "read_to_string" function of the "file" handle is used to read contents of that file into a string variable, passed by reference to a mutable object.
```rust
use std::io::Read;

fn main(){

    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}

```

## Copy a file

Let us copy contents of a file and create a new file as shown below

```rust
use std::io::Read;
use std::io::Write;

fn main(){
 
        let mut command_line: std::env::Args = std::env::args();
        command_line.next().unwrap();// skip the executable file name

        let source = command_line.next().unwrap(); // take the source file
        let destination = command_line.next().unwrap(); // take the destination file

        let mut file_in = std::fs::File::open(source).unwrap();
        let mut file_out = std::fs::File::create(destination).unwrap();
        let mut buffer = [0u8; 4096];
        loop {
            let nbytes = file_in.read(&mut buffer).unwrap();
            file_out.write(&buffer[..nbytes]).unwrap();
            if nbytes < buffer.len() { break; }
        }
}

```
Execute the above program as *main.exe data.txt datacopy.txt* . Two command line agruments are passed while executing the file- the path to the source file and the destination file respectively.

## Modify contents..
The lines from the third to the sixth one assign to the "source" variable the contents of the first argument, and to the "destination" variable the contents of the second argument.
The next two lines open the two files. First the source file is opened, and the new handle is assigned to the "file_in" variable. Then the destination file is created (or truncated, if already existing), and the new handle is assigned to the "file_out" variable.
Then a 4096-byte buffer is allocated in the stack.
At last, a loop repeatedly reads a 4096-byte chunk from the source file and writes it to the output file. The number of bytes read is automatically as many as the length of the buffer

For a file larger than 4096 bytes, at the first iteration the number of bytes read will be 4096, and so some other iterations will be required. For a smaller file, one iteration will be enough. 
In any case, the buffer is written to the file up to the number of bytes read. So, a slice of the buffer is taken from the beginning to the number of read bytes
