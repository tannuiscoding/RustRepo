### how to set up your first project 

run `cargo new hello_world` to create a directory and then `cd hello_world` to open get into the directory. In the directory there would be a `src` folder and `Cargo.toml` file.

### Your first rust program



## You may ask 
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

> We assume you know basics and how to install anything on your speicified OS, so no tutorial for that

### Day 1 checklist