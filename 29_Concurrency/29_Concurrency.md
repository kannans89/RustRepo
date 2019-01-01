
# Concurrency

In Concurrent programming,  different parts of a program execute independently.On the other hand,in parallel programming,  different parts of a program execute at the same time. Both models are becoming increasingly important as more computers take advantage of their multiple processors.

## Threads

We can use threads to run code simultaneously.In most current operating systems, an executed program’s code is run in a process, and the operating system manages multiple processes at once. Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.

### Creating a Thread

The thread::spawn function is used to create a new thread.The spawn function takes a closure as parameter. The closure defines code that should be executed by the thread.The following example prints some text from a main thread and other text from a new thread.

```rust
//import the necessary modules
use std::thread;
use std::time::Duration;

fn main() {

//create a new thread
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });


//code executed by the main thread
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

```

Output

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

The main thread prints values from **1 to 4** .Note that the new thread will be stopped when the main thread ends, whether or not it has finished running. The output from this program might be a little different every time,


The thread::sleep function forces a thread to stop its execution for a short duration, allowing a different thread to run. The threads will probably take turns, but that isn’t guaranteed: it depends on how the operating system schedules the threads. In this run, the main thread printed first, even though the print statement from the spawned thread appears first in the code. And even though the spawned thread is programmed to print values till 9, it only got to 5 before the main thread shut down.

## Join Handles

As discussed, a spawned thread may not get a chance to run or run completely to run.This is because the main thread completes quickly.The function `spawn<F, T>(f: F) -> JoinHandle<T> ` returns a JoinHandle.The `join()` method on JoinHandle waits for the associated thread to finish.
<!-- //appu: make the explanation simple
by saving the return value of thread::spawn in a variable. The return type of thread::spawn is JoinHandle. A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish. -->

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

Output of the program will be as  shown

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

From the output it is clear that main thead and spawned thread  continue switching. Note the main thread waits for spawned thread to complete because of the call to the join() method.