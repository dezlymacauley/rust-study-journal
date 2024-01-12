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





