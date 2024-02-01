#  Rust Study Journal

### Current progress 

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


### Completed
- The Rust Programming Language
- Foreword
- Introduction

#### 1. Introduction (Currently Studying)



---

#### How to use cargo (Rust's package manager and build system)

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
