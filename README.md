# RustRepo

A comprehensive 21-day Rust learning journey with detailed notes, code examples, and practice exercises.

## Overview

This repository contains my learning progress through Rust programming language fundamentals. Each day focuses on specific concepts with practical examples, detailed notes, and hands-on exercises.

## Daily Progress

### [Day 1: Getting Started with Rust](./day_1/notes.md)
**Topics Covered:**
- Setting up your first Rust project with Cargo
- Understanding `fn` keyword and `main()` function
- Working with `println!()` macro vs functions
- Introduction to Cargo and `Cargo.toml` structure
- Access modifiers (`pub` keyword)
- Modules (`mod` keyword)
- Mutable vs immutable variables
- User input basics (`std::io`, `.read_line()`, `.trim()`, `.parse()`)
- Conditional statements (`if`, `else if`, `else`)
- **Project:** [Calculator Application](./day_1/src/)

### [Day 2: Data Types & Variables](./day_2/notes.md)
**Topics Covered:**
- Scalar data types (integers, floats, booleans, characters)
- Compound data types (tuples, arrays)
- Type safety and static typing
- Variables and mutability (`let`, `mut`)
- Borrowing concepts (`&T`, `&mut T`)
- Shared vs mutable borrows
- Memory safety and compile-time checks
- **Practice:** [Data Types Exercises](./day_2/practice_que.md)

## Complete Topic Index

### Fundamentals
- [x] Project Setup & Cargo (`cargo new`, `Cargo.toml`, `cargo run`)
- [x] Basic Syntax (`fn`, `main()`, macros vs functions)
- [x] Variables & Mutability (`let`, `mut`, immutability by default)

### Data Types
- [x] Scalar Types (integers, floats, booleans, characters)
- [x] Compound Types (tuples, arrays)
- [x] Type safety and static typing

### Control Flow & I/O
- [x] Conditionals (`if`, `else if`, `else`)
- [x] User Input (`std::io`, `read_line()`, `parse()`)

### Memory Management
- [x] Borrowing System (`&T`, `&mut T`, borrowing rules)
- [x] Modules (`mod`, `pub`, cross-module calls)

## �� Getting Started

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd RustRepo
   ```

2. **Navigate to any day's project:**
   ```bash
   cd day_1  # or day_2, etc.
   ```

3. **Run the code:**
   ```bash
   cargo run
   ```

4. **Read the notes:**
   - Each day has a `notes.md` file with detailed explanations
   - Practice questions are available in `practice_que.md` files

## 📖 Learning Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Reference](https://doc.rust-lang.org/reference/)

## 🎯 Project Goals

- [ ] Complete 21 days of Rust learning
- [ ] Build practical applications
- [ ] Understand Rust's memory safety model
- [ ] Master ownership and borrowing concepts
- [ ] Practice with real-world examples

## Notes

- All code examples are tested and working
- Notes include explanations, code snippets, and practice questions
- Each day builds upon previous concepts
- Focus on understanding rather than memorization

---

**Happy Rusting! 🦀**
