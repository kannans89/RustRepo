
# Environment Setup

- Installation link for windows   [Rust Lang](https://www.rust-lang.org/en-US/install.html)

- Download the **rustup-init.exe**

- After download double click on the **rustup-init**  file following window appears.

```javascript
This will download and install the official compiler for the Rust programming
language, and its package manager, Cargo.

It will add the cargo, rustc, rustup and other commands to Cargo's bin
directory, located at:

  C:\Users\Admin\.cargo\bin

This path will then be added to your PATH environment variable by modifying the
HKEY_CURRENT_USER/Environment/PATH registry key.

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

- VSCode extensions to be installed is **Rust (rls)** .This adds code completion,code formatting,refactoring features for Rust in Visual Studio Code .

<!--
  External References for this chapter
  
  1. https://www.youtube.com/watch?v=EKJi8BCoynY

  2. https://booyaa.wtf/2017/rust-vscode/index.html

 -->