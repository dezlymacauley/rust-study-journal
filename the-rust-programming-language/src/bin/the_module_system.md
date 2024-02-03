
Rust has a number of features,
that allow you to manage your code’s organization, 
including which details are exposed, which details are private, 
and what names are in each scope in your programs.

These features, sometimes collectively referred to as the module system,
include:

   - Packages: A Cargo feature that lets you build, test, and share crates
   - Crates: A tree of modules that produces a library or executable
   - Modules and use: Let you control the organization, scope, and privacy of paths
   - Paths: A way of naming an item, such as a struct, function, or module

To create a new project:

`cargo new name-of-your-project`

Cargo follows a convention that src/main.rs, 
is the crate root of a binary crate with the same name as the package. 

Likewise, Cargo knows that if the package directory contains src/lib.rs,
the package contains a library crate with the same name as the package,
and src/lib.rs is its crate root.

Cargo passes the crate root files to rustc to build the library or binary.

If a package contains src/main.rs and src/lib.rs,
it has two crates: a binary and a library,
both with the same name as the package.

VERY IMPORTANT:
A package can have multiple binary crates,
by placing files in the src/bin directory: each file will be a separate binary crate.

To run a specific file 
`cargo run --bin the-name-of-the-file-excluding-the-rs-part`
E.g. `cargo run --bin 6_functions`



