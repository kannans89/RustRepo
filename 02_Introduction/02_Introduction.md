# Introduction

Rust is a systems level programming language ,developed by *Graydon Hoare* and was taken over later by Mozilla Labs.

## Application v/s Systems Programming Languages

Application programming languages like Java/C# are used to build software which provide services to the user directly.They help us build business applications like spread sheets , word processors , web applications or mobile apps.

Systems programming languages like C/C++ are used to build software and software platforms. They can be used to build operating systems, game engines ,compilers etc. These  programming languages require a great degree of hardware interaction.

Systems and application programming languages face two major problems:

1. It is difficult to write secure code.
2. It is difficult to  write multi-threaded code.

## Why Rust

 Rust focuses on three goals **safety**,**speed** and **concurrency**.
 It is designed for developing highly reliable and fast software in a simple way. It can be used to write high-level programs down to hardware-specific programs.

 1. **Performance**: Rust programming language doesn't have a Garbage Collector(GC) by design.This improves the performance at runtime .
 2. **Memory safety at compile time** :Software built using Rust are safe from memory issues like dangling pointers,buffer overruns and memory leaks.
 3. **Multi threaded applications** :Rust's ownership and memory safety rules provide concurrency without data races.
 4. **Support for WebAssembly(Wasm)**: WebAssembly helps to execute high computation intensive algorithms in the browser, on embedded devices, or anywhere else. It runs at the speed of native code . Rust can be compiled to WebAssembly for fast, reliable execution.

<!-- External links for this chapter
 //day2
 1. good intro : https://stackoverflow.com/tags/rust/info
 2. projects using rust : https://en.wikipedia.org/wiki/Rust_(programming_language)#Projects_using_Rust
 3. system prgramming : https://en.wikipedia.org/wiki/System_programming_language
 4. where is it used /more confidence: https://medium.com/mozilla-tech/why-rust-is-the-most-loved-language-by-developers-666add782563

 //day1
 1. https://www.quora.com/What-is-your-review-of-the-Rust-programming-language

 2. from mozilla to apple https://www.reddit.com/r/rust/comments/7qels2/i_wonder_why_graydon_hoare_the_author_of_rust/
 
 3. code samples :https://rust-lang-nursery.github.io/rust-cookbook/

 4. slowgrammer :https://github.com/rust-lang-nursery

 5. twitter: https://twitter.com/softprops

 6. graydon handle: https://graydon2.dreamwidth.org/247406.html

 7. Rust is more than safety: https://words.steveklabnik.com/rust-is-more-than-safety

    - null pointes/options:https://insanitybit.github.io/2016/12/28/why-rust-sum-types
    - rust in 2017 :https://medium.com/@Hisako1337/rust-in-2017-8f2b57a67d9b
      1.unsafe rust: https://doc.rust-lang.org/nomicon/ 

8. Programming wikipedia : http://progopedia.com/language/rust/

-->
