
# Environment Setup

Installation of Rust is made easy through **rustup**, a console based tool for managing Rust versions and associated tools.

## Installation on Windows

- Installation of Visual Studio 2013 or higher with C++ tools is mandatory to run the Rust program on windows.First download Visual Studio from here [VS 2013 Express] (http://download.microsoft.com/download/2/5/5/255DCCB6-F364-4ED8-9758-EF0734CA86B8/vs2013.3_dskexp_ENU.iso )

- Download and install rustup tool for windows .**rustup-init.exe** is available for download here-[Rust Lang](https://www.rust-lang.org/en-US/install.html)

- Double click on the **rustup-init.exe**  file. the following screen will  appear.

![installation_screen](https://user-images.githubusercontent.com/9062443/49558134-073d3b80-f930-11e8-8e4f-85af4004ed15.png)

- Press `enter`key to do default installation . Once installation is completed following screen appears.

![installation_screen2](https://user-images.githubusercontent.com/9062443/49558030-a9105880-f92f-11e8-8ee8-7116034ba5ec.png)

From the installation screen it is clear that Rust related files are stored in the following folder `C:\Users\{PC}\.cargo\bin`
Following are contents of the folder.

```javascript
cargo-fmt.exe
cargo.exe
rls.exe
rust-gdb.exe
rust-lldb.exe
rustc.exe  // this is the compiler for rust
rustdoc.exe
rustfmt.exe
rustup.exe
```

- **cargo** is the package manager for Rust.  To verify if **cargo** is installed execute the following command:

```javascript
C:\Users\Admin>cargo -V

cargo 1.29.0 (524a578d7 2018-08-05)

```

- The compiler for Rust is **rustc**. To verify the compiler version , execute the follwing command:

```javascript
C:\Users\Admin>rustc --version
rustc 1.29.0 (aa3ca1994 2018-09-11)

```

## Installation on Linux / Mac

 To install **rustup** on Linux or macOS open a terminal and enter following 
 command

 ```rust
  $ curl https://sh.rustup.rs -sSf | sh
 ```
The command downloads a script and starts the installation of the rustup tool, which installs the latest stable version of Rust. You might be prompted for your password. If the installation is successful, the following line will appear:

`Rust is installed now. Great!`

The installation script automatically adds Rust to your system PATH after your next login. If you want to start using Rust right away instead of restarting your terminal, run the following command in your shell to add Rust to your system PATH manually:

`$ source $HOME/.cargo/env`

Alternatively, you can add the following line to your ~/.bash_profile:

`$ export PATH="$HOME/.cargo/bin:$PATH"`

Note that when you try to compile a Rust program and get errors indicating that a linker could not execute, that means a linker isn’t installed on your system and you’ll need to install one manually.

C compilers usually come with the correct linker. Check your platform’s documentation for how to install a C compiler. Also, some common Rust packages depend on C code and will need a C compiler. Therefore, it might be worth installing one .

## Using TutorialsPoint *Coding Ground for RUST*

A Read-Evaluate-Print Loop (REPL) is an easy to use interactive shell to compile and execute computer programs. If you want to compile and  execute Rust programs online within the browser use TutorialsPoint [**Coding Ground** ](https://www.tutorialspoint.com/compile_rust_online.php)

<!--
  External References for this chapter
  
  1. https://www.youtube.com/watch?v=EKJi8BCoynY

  2. https://booyaa.wtf/2017/rust-vscode/index.html

 -->
