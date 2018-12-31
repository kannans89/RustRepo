# File Input/Output

In addition to reading and writing to console,Rust allows reading and writing to files.

The File struct represents a file.It allows a program to perform read-write  operations on a file.All methods in the File struct return a variant of the io::Result<T> enumeration.

The commonly used methods of the File struct are:

|Sr No |module| method | signature   | description|  
|:----:|:---|:----------|:-------|:-------|
| 1    | std::fs::File |open()  | pub fn open<P: AsRef<Path>>(path: P) -> Result<File>|The open static method can be used to open a file in read-only mode
| 2    | std::fs::File |create()  |pub fn create<P: AsRef<Path>>(path: P) -> Result<File> |static method opens a file in write-only mode. If the file already existed, the old content is destroyed. Otherwise, a new file is created.
| 3    | std::fs::remove_file |remove_file() |pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<()>|Removes a file from the filesystem,there is no guarantee that the file is immediately deleted
|4|std::fs::OpenOptions|append()|pub fn append(&mut self, append: bool) -> &mut OpenOptions|Sets the option for the append mode of file
|5|std::io::Write|write_all()|fn write_all(&mut self, buf: &[u8]) -> Result<()>|Attempts to write an entire buffer into this write
|6|std::io::Read|read_to_string()|fn read_to_string(&mut self, buf: &mut String) -> Result<usize>|Read all bytes until EOF in this source, appending them to buf.

## Write to a File

Let us understand this with an example.

The following program creates a file 'data.txt'.The create() method is used to create a file. The method returns a file handle if the file is created successfully.The last line *write_all* function will write bytes in newly created file.If the any of the operations fail, the expect() function returns an error message.

```rust

use std::io::Write;
fn main(){
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Hello World".as_bytes()).expect("write failed");
    file.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");

    println!("data written to file" );
}
```

output : `data written to file`

## Read from a File

The following program reads the contents in a file *data.txt* and prints it to the console.
The "open" function is used to open an existing file. An absolute or relative path to the file is passed to the open() function as a parameter.
The open() function throws an exception if the file does not exist, or if it is not accessible for whatever reason. If it succeeds, a file handle to such file is assigned to the "file" variable.

The "read_to_string" function of the "file" handle is used to read contents of that file into a string variable.

```rust
use std::io::Read;

fn main(){

    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}

```

output:

```rust
Hello World
TutorialsPoint
```

## Delete a file

The following example uses the remove_file() function to delete a file. The expect() function returns a custom message in case an error occurs.

```rust
use std::fs;

fn main(){
    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");
}

```

Output:

```rust
file is removed
```

## Append data to a file

The append() functions writes data to the end of the file. This is shown in the example given below:

```rust
use std::fs::OpenOptions;
use std::io::Write;

fn main(){
  let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
  file.write_all("Hello World".as_bytes()).expect("write failed");
  file.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");
  println!("file append success");
}

```

output: `file append success`

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

Execute the above program as *main.exe data.txt datacopy.txt* . Two command line arguments are passed while executing the file- the path to the source file and the destination file respectively.

<!--
 Modify contents..
The lines from the third to the sixth one assign to the "source" variable the contents of the first argument, and to the "destination" variable the contents of the second argument.
The next two lines open the two files. First the source file is opened, and the new handle is assigned to the "file_in" variable. Then the destination file is created (or truncated, if already existing), and the new handle is assigned to the "file_out" variable.
Then a 4096-byte buffer is allocated in the stack.
At last, a loop repeatedly reads a 4096-byte chunk from the source file and writes it to the output file. The number of bytes read is automatically as many as the length of the buffer

For a file larger than 4096 bytes, at the first iteration the number of bytes read will be 4096, and so some other iterations will be required. For a smaller file, one iteration will be enough. 
In any case, the buffer is written to the file up to the number of bytes read. So, a slice of the buffer is taken from the beginning to the number of read bytes
-->