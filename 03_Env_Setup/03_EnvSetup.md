
# Environment Setup

Installation of Rust is made easy through **rustup**, a command line tool for managing Rust versions and associated tools. You’ll need an internet connection for the download.

## Installation on Windows

Installation of visual studio 2013 or higher with C++ tools is mandatory to run the rust program on windows.You can download here [VS 2013 Express](http://download.microsoft.com/download/2/5/5/255DCCB6-F364-4ED8-9758-EF0734CA86B8/vs2013.3_dskexp_ENU.iso )

- Installation link for windows   [Rust Lang](https://www.rust-lang.org/en-US/install.html)

- Download the **rustup-init.exe**

- After download double click on the **rustup-init**  file following window appears.

```javascript
This will download and install the official compiler for the Rust programming
language, and its package manager, Cargo.

It will add the cargo, rustc, rustup and other commands to Cargo's bin
directory, located at:

  C:\Users\Admin\.cargo\bin

This path will then be added to your PATH environment variable.

You can uninstall at any time with rustup self uninstall and these changes will
be reverted.

Current installation options:

   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>

```

- Press `enter`key to do default installation . Once installation is completed following screen appears.

```javascript
info: syncing channel updates for 'stable-x86_64-pc-windows-msvc'
info: latest update on 2018-09-13, rust version 1.29.0 (aa3ca1994 2018-09-11)
info: downloading component 'rustc'
 51.5 MiB /  51.5 MiB (100 %) 582.4 KiB/s ETA:   0 s
info: downloading component 'rust-std'
 45.1 MiB /  45.1 MiB (100 %) 684.8 KiB/s ETA:   0 s
info: downloading component 'cargo'
  2.7 MiB /   2.7 MiB (100 %) 704.0 KiB/s ETA:   0 s
info: downloading component 'rust-docs'
  8.2 MiB /   8.2 MiB (100 %) 307.2 KiB/s ETA:   0 s
info: installing component 'rustc'
info: installing component 'rust-std'
info: installing component 'cargo'
info: installing component 'rust-docs'
info: default toolchain set to 'stable'

  stable installed - rustc 1.29.0 (aa3ca1994 2018-09-11)


Rust is installed now. Great!

To get started you need Cargo's bin directory (%USERPROFILE%\.cargo\bin) in
your PATH environment variable. Future applications will automatically have the
correct environment, but you may need to restart your current shell.


```

From the installation screen it is clear that rust related files are stored in the following folder `C:\Users\Admin\.cargo\bin`
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

- After installation is succeeded . You can check the **cargo** package manager through terminal as given below

```javascript
C:\Users\Admin>cargo -V

cargo 1.29.0 (524a578d7 2018-08-05)

```

- Verify the compiler version in terminal as below

```javascript
C:\Users\Admin>rustc --version
rustc 1.29.0 (aa3ca1994 2018-09-11)

```

## Installation on Linux / Mac

 To install rustup on Linux or macOS open a terminal and enter following 
 command

 ```rust
  $ curl https://sh.rustup.rs -sSf | sh
 ```
The command downloads a script and starts the installation of the rustup tool, which installs the latest stable version of Rust. You might be prompted for your password. If the install is successful, the following line will appear:

`Rust is installed now. Great!`

The installation script automatically adds Rust to your system PATH after your next login. If you want to start using Rust right away instead of restarting your terminal, run the following command in your shell to add Rust to your system PATH manually:

`$ source $HOME/.cargo/env`

Alternatively, you can add the following line to your ~/.bash_profile:

`$ export PATH="$HOME/.cargo/bin:$PATH"`

Note that when you try to compile a Rust program and get errors indicating that a linker could not execute, that means a linker isn’t installed on your system and you’ll need to install one manually.

C compilers usually come with the correct linker. Check your platform’s documentation for how to install a C compiler. Also, some common Rust packages depend on C code and will need a C compiler. Therefore, it might be worth installing one .

## Using TutorialsPoint RUST REPL

A Read-Evaluate-Print Loop (REPL) is an easy to use interactive shell to compile and execute computer programs . If you want to compile and  execute Rust programs online within the browser use TutorialsPoint [**coding ground** ](https://www.tutorialspoint.com/compile_rust_online.php)

<!--
  External References for this chapter
  
  1. https://www.youtube.com/watch?v=EKJi8BCoynY

  2. https://booyaa.wtf/2017/rust-vscode/index.html

 -->