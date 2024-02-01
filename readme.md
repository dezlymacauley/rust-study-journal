#  Rust Study Journal

### Current progress 

---

### Completed

##### The Rust Programming Language
##### Foreword

##### Introduction
##### 1. Introduction ##### 1.1 Installation
##### 1.2 Hello, World!
##### 1.3 Hello, Cargo!

##### 3. Common Programming Concepts 
##### 3.1 Variables and Mutability 
##### 3.2 Data Types (Currently Studying)
##### 3.4 Comments

---

"The Book"

NOTE: If you have Rust installed on your system, 
you can access an offline copy of the book with the following command:

`rustup docs --book`

Project Chapters: 2, 12, 20

" If you’re a particularly meticulous learner, 
who prefers to learn every detail before moving on to the next,
you might want to skip Chapter 2 and go straight to Chapter 3,
returning to Chapter 2 when you’d like to work on a project, 
applying the details you’ve learned."

---

### How to install Rust (Arch Linux)

First install the Rust toolchain installer.

`sudo pacman -S rustup`


Next you need to install a Linker:

This is a program that Rust uses to join its compiled outputs into one file.

The easiest way to get a Linker is to install C compiler, 
which will typically include a linker.

A C compiler is also useful because some common Rust packages,
depend on C code and will need a C compiler.

There are many ways to get a C compiler.

Feel free to install one of the ones below, 
all all of them just to avoid any issues in the future.

`sudo pacman -S base-devel`

`sudo pacman -S gcc`

`sudo pacman -S clang`


To check that it was installed correctly

`rustc --version`

Make sure that Rust has been added to your path:

`echo $PATH`

Look for a something that looks like this:

/usr/lib/rustup/bin

If you see that then Rust has been added to your path.


Next install rust-analyzer (Languages Support in your Code Editor of choice):

`rust-analyzer --version`

Each version of Rust will have its own Rust Analyzer.
If your current version of Rust doesn't have Rust Analyzer, 
then you will be given a command to install it.

---

### How to use rustc 

For running simple one page programs

rustc name_of_rust_file.rs

E.g. `rustc cool_project.rs`

This will create a binary with the same name:

cool_project


To run the binary `./cool_project`


Note: You can additional flags when compiling your code.

`rustc cool_project.rs -g` The -g flag will create a binary for debugging

`rustc --target=x86_64-apple-darwin cool_project.rs` Create binary for MacOS

`rustc --target=x86_64-pc-windows-msvc cool_project.rs` Create binary for Windows

`rustc --target=x86_64-unknown-linux-gnu main.rs` Create binary for Linux

For Linux: 
the default target architecture is usually determined by the host system,
and you typically don't need to specify a target explicitly.


Rust is an ahead-of-time compiled language,
meaning you can compile a program and give the executable to someone else,
and they can run it even without having Rust installed.

---

#### How to use cargo (Rust's package manager and build system)

For running more complex Rust programs:

WAIT: First make sure that your project has a .gitignore file

https://github.com/github/gitignore/blob/main/Rust.gitignore



`cargo new name-of-project`

`cargo build` This will create a binary executable of your project.

`cargo run` This will create a binary executable and run it.

`cargo check` This will check if your code compiles but does not produce,
and executable.

`cargo build --release` This will create a release version of your project.
This binary will take longer to create, 
because it includes some opimizations that will make your code run faster. 

---

#### How to build from an existing project:

In fact, to work on any existing projects, you can use the following commands to check out the code using Git, change to that project’s directory, and build:

`git clone example.org/someproject`
`cd someproject`
`cargo build`

---

One of my main motivations for learning Rust.

"Working with Rust allows you to build skills that transfer from one domain,
to another; you can learn Rust by writing a web app, 
then apply those same skills to target your Raspberry Pi." - Foreword

---

Rust Developer Tools:



Cargo, the included dependency manager and build tool, makes adding, compiling, and managing dependencies painless and consistent across the Rust ecosystem.
The Rustfmt formatting tool ensures a consistent coding style across developers.
The Rust Language Server powers Integrated Development Environment (IDE) integration for code completion and inline error messages.

---

### Terminology 

Zero-cost Abstractions:

""Zero-cost abstractions" is a concept often associated with programming languages like C++ and Rust. It refers to the idea that using high-level abstractions in your code, such as functions, classes, or other constructs that make your code more readable and maintainable, should not result in any runtime performance penalty."
In simpler terms, when you write code using these abstractions, the compiler should be able to optimize and generate machine code that is as efficient as if you had written the lower-level code manually. The goal is to provide developers with the benefits of abstraction without sacrificing runtime performance.

This concept is crucial in systems programming, where performance is often a top priority. It allows developers to write code at a higher level of abstraction for better readability and maintainability, while still achieving the efficiency of low-level, manual implementations.

Languages like C++ and Rust aim to provide zero-cost abstractions by using sophisticated compilers that optimize the code during the compilation process, eliminating the need for developers to choose between abstraction and performance.

---

Concurrency Bugs:

Concurrency bugs are issues that arise in a program when multiple threads or processes execute simultaneously and interact with shared resources. Concurrency bugs can be challenging to identify and fix because they often depend on the specific timing and interleaving of operations in a concurrent execution.

Here are some common types of concurrency bugs:

    Race Conditions: Race conditions occur when the behavior of a program depends on the timing or interleaving of multiple threads. For example, if two threads access a shared variable concurrently, and at least one of them modifies it, the final state of the variable depends on which thread finishes first.

    Deadlocks: A deadlock happens when two or more threads are blocked forever, each waiting for the other to release a lock. This can occur when multiple threads acquire locks in a different order, and a cycle is formed.

    Starvation: Starvation occurs when a thread is unable to gain access to a shared resource or lock because other threads consistently take precedence. This can lead to a situation where a thread is delayed indefinitely.

    Data Races: Data races occur when two or more threads access the same memory location concurrently, and at least one of them is a write operation. The behavior of the program becomes unpredictable in the presence of data races.

    Atomicity Violations: Atomicity issues arise when an operation that is supposed to be atomic is interrupted by another thread. For example, if an increment operation on a shared variable is not atomic, it may be interrupted by another thread, leading to incorrect results.

    Concurrency bugs can be challenging to debug because they often manifest themselves sporadically and are dependent on the specific timing of events. Tools like static analyzers, dynamic analysis tools, and proper synchronization mechanisms (such as locks, semaphores, and atomic operations) can help mitigate and identify concurrency bugs. Additionally, writing thread-safe code and following best practices for concurrent programming can reduce the likelihood of encountering concurrency issues.

---
Look into how to use hashmaps in Rust

What are `unit functions`?

These are functions that are just executing some code.
Unit functions are not returning anything

Teachers:
Alfredo Sanchez

TODO: 
Attempt the File reader application from the `Function Basics` section

Go over `associated functions and constructors` from the section,
`Using structured data`

Look up `Idiomatic Rust`

Look up CodeLLBD Debugger


README.md = Intial project description
LICENCE = Project files
Cargo.toml = Lists the package and its dependencies


Cargo.lock = Locks in the versions of those packages. 
This is something that you would do for binary packages.
For library packages, you want to keep things flexible.

`src` folder = source
main.rs = For binary packages, command line tools
lib.rs = A file that has additional modules

Make file (This will be covered later)


## How to create a new binary package:

If you run the command `cargo init .` it will create a new Rust project, 
using the current directory as the project name.

You can also run `cargo init --help`

---

## How to reset a project (delete files):

Remove the  `Cargo.toml` file and the `src` folder:
`rm -f Cargo.toml`

`rm -f src/`

---

## How to create a new library package:
`cargo init --lib .`

---

# How to create a new project, in a new directory

For a binary package, do this:
`cargo new name-of-new-folder`

For a library package, do this:
`cargo new --lib name-of-new-folder`


Use this for help
`cargo new --help`


---





