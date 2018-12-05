
# What is Concurrency?

In Concurrent programming,  different parts of a program execute independently.In parallel programming,  different parts of a program execute at the same time. Both models are becoming increasingly important as more computers take advantage of their multiple processors. Historically, programming in these contexts has been difficult and error prone: Rust hopes to change that.By leveraging ownership and type checking,many concurrency errors are compile-time errors in Rust rather than runtime errors.

## Threads

We can use threads to run code simultaneously.In most current operating systems, an executed program’s code is run in a process, and the operating system manages multiple processes at once. Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.

### Creating a New Thread with spawn

To create a new thread, we call the thread::spawn function and pass it a closure containing the code we want to run in the new thread. The example following  prints some text from a main thread and other text from a new thread.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}



```

output is shown below

```rust
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 4 from the main thread!

```

From the output it is clear that only all iterations of main thread is executed . The priting operation inside main thread is prited from **1 to 4** .
Note that with this function, the new thread will be stopped when the main thread ends, whether or not it has finished running. The output from this program might be a little different every time,


The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run. The threads will probably take turns, but that isn’t guaranteed: it depends on how your operating system schedules the threads. In this run, the main thread printed first, even though the print statement from the spawned thread appears first in the code. And even though we told the spawned thread to print until i is 9, it only got to 5 before the main thread shut down

## Join Handles

he problem of the spawned thread not getting to run, or not getting to run completely, by saving the return value of thread::spawn in a variable. The return type of thread::spawn is JoinHandle. A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish.

```rust
use std::thread;
use std::time::Duration;

fn main() {
   let handle= thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
   handle.join().unwrap();
}


```

output of the program will be as  shown

```rust

hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 2 from the main thread!
hi number 3 from the spawned thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!


```

Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates. Blocking a thread means that thread is prevented from performing work or exiting. Because we’ve put the call to join after the main thread’s for loop, running Listing 16-2 should produce output similar to this:

The two threads continue alternating, but the main thread waits because of the call to handle.join() and does not end until the spawned thread is finished.