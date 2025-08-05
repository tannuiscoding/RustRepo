# how to set up your first project 

run `cargo new hello_world` to create a directory and then `cd hello_world` to open get into the directory. In the directory there would be a `src` folder and `Cargo.toml` file.

### Your first rust program

```rust
fn main(){
    println!("Hello world");
}
```

## let's break it down

- `fn` is a keyword used to define functions
- `main()` is the main  function that is executed first and foremost like other languages C++, Java and C
- `println!()` is a macro (works like function but not function) that is used to print on the console

# You may ask 
- What is Cargo?
  - Cargo is a built-in package manager and build system for Rust, similar to `npm` for Node.js.  
    It manages dependencies, compiles code with `rustc`, runs tests, builds documentation, and packages your project.  
    When you use commands like:
    ```bash
    cargo run
    cargo build
    ```
    Cargo reads your project configuration, resolves dependencies, and compiles your code.

- What content does `Cargo.toml` have?
  - **`[package]`** – Project metadata *(name, version, edition, authors, license, description, repository, readme, etc.)*
  - **`[dependencies]`** – Crates (libraries) required for the project
  - **`[dev-dependencies]`** – Crates required only for tests or benchmarks
  - **`[build-dependencies]`** – Crates needed for build scripts (`build.rs`)
  - **`[features]`** – Optional compile-time features to enable/disable code sections
  - **`[profile]`** – Compiler settings for `debug` and `release` builds
  - **`[workspace]`** – Workspace configuration for multi-package projects
  - **`[patch]`** – Override a dependency with a different source
  - **`[replace]`** *(deprecated)* – Old way to override dependencies
  - **`[target]`** – Target-specific dependency configurations (e.g., `wasm32`, `x86_64`)
  - **`[badges]`** – Metadata for displaying badges on crates.io (e.g., CI status)

- what is a macro?
  - macros are like functions but defined with and `!` exclamation mark. it can do the work that functions can't like accepting a variable as an argument.
- can I use `println()` ?
  - no bro, you can't, it will throw an error. ```error: cannot find function println in this scope```

## Questions you may ask if you have read the code

- what are mutable variables?
  - you can change the values later on or in easy words, assign the values again.

- what is keyword `pub`?
  - this keyword is the access modifier, means public. if you want to use this function in other file, you will have to make your functions public.

- what does keyword `mod` means?
  - stands for module, you use this keyword when you are using another functions from another module.

- are the variables mutable by default?
  - no, they are immutable by default.

- Why does Rust make variables immutable by default? Give two reasons.
  - Prevents accidental modification of data.
  - Improves safety and allows the compiler to optimize better.

- What happens if `.parse()` fails to convert a string into a number?
  - the program will panic if `.expect()` is used, showing an error message.

- Can you write if (x > 5) in Rust with parentheses? Will it work?
  - yes, parentheses are allowed but optional; Rust does not require them.

> We assume you know basics and how to install anything on your speicified OS, so no tutorial for that

### Day 1 checklist

- [x] Understood:
  - `fn` keyword for functions
  - `main()` function entry point
  - `println!()` macro usage
  - Difference between macros and functions
- [x] Learned about **Cargo** and its role (dependency management, building, running)
- [x] Explored **Cargo.toml** structure and key sections (`[package]`, `[dependencies]`, etc.)
- [x] Learned about:
  - `pub` keyword (public access modifier)
  - `mod` keyword (modules)
  - mutable vs immutable variables
  - why variables are immutable by default
- [x] learned user input basics (`std::io`, `.read_line()`, `.trim()`, `.parse()`)
- [x] practiced conditionals (`if`, `else if`, `else`) with and without parentheses
- [x] made a calculator