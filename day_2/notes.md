# Index – Day 2: Data Types & Variables in Rust

> A quick overview of all the key topics covered on Day 2 of the 21 Days of Rust Challenge.

---

## Topics Covered

1. **Introduction to Data Types**
   - Importance of type safety in Rust
   - Static typing and compile-time checks

2. **Scalar Data Types**
   - Integer types (`i8`, `u8`, ..., `i128`, `u128`, `isize`, `usize`)
   - Floating-point types (`f32`, `f64`)
   - Boolean (`bool`)
   - Character (`char`)

3. **Compound Data Types**
   - Tuples
     - Creating and destructuring
     ```rust
     let tup = (500, 6.4, 1);
     let (x, y, z) = tup;    // Destructuring
     let first = tup.0;      // Access via dot notation
     ```
   - Arrays
     - Fixed-size and same-type
     ```rust 
     let arr = [1, 2, 3, 4, 5];
     let first = arr[0];     // Access via index
     let second = arr[1];
     ```

4. **Tuple vs Array Comparison**
   - Differences in use cases, typing, and syntax

5. **String**  
   - String types: `String` vs `&str`
   - Option and Result types
   - Variables: `let`, `mut`, shadowing

## Data Types in Rust

1. **Scalar Data Types**: Hold single values
    - **Integer Types**:
        - Signed integers: `i8` to `i128` (e.g., `i8`, `i16`, `i32`, `i64`, `i128`)
        ```rust
        let x: i32 = -42;
        let y: i8 = 127;  // max value for i8
        ```
        - Unsigned integers: `u8` to `u128` (e.g., `u8`, `u16`, `u32`, `u64`, `u128`)
        ```rust
        let age: u8 = 25;
        let big_num: u64 = 18446744073709551615;  // max value for u64
        ```
        - Range for signed integers: -2^(n-1) to 2^(n-1)-1 where n is bits
        - Architecture-specific: `isize`, `usize` (matches system architecture)
        ```rust
        let array_size: usize = 10;
        let pointer_sized: isize = -300;
        ```
    
    - **Floating-Point Types**:
        - `f32`: Single precision (32-bit)
        - `f64`: Double precision (64-bit, default)
        ```rust
        let pi: f64 = 3.14159265359;
        let e: f32 = 2.71828;
        let default_float = 3.14; // f64 by default
        ```
        - IEEE-754 standard compliant
    
    - **Boolean Type**:
        - `bool`: Can be either `true` or `false`
        ```rust
        let is_rust_fun: bool = true;
        let is_basic: bool = false;
        ```
        - Used in conditional statements and logical operations
    
    - **Character Type**:
        - `char`: Unicode scalar value
        ```rust
        let letter: char = 'A';
        let emoji: char = '😀';
        let heart_symbol: char = '♥';
        ```
        - 4 bytes in size
        - Represents Unicode characters including emojis and special symbols

2. **Compound Data Types**: Group multiple values
    - **Tuples**:
        - Fixed-length ordered collection
        - Can hold multiple types
        ```rust
        let tuple: (i32, f64, char) = (42, 3.14, 'A');
        let (x, y, z) = tuple; // destructuring
        let first = tuple.0;   // accessing via index
        ```
        - Created using parentheses: `(value1, value2, ...)`
        - Access via dot notation and zero-based indexing (.0, .1, etc.)
        - Can be destructured: `let (x, y, z) = tuple;`
    
    - **Arrays**:
        - Fixed-length collection of same-type elements
        ```rust
        let numbers: [i32; 5] = [1, 2, 3, 4, 5];
        let first_num = numbers[0];
        let zeros = [0; 10]; // creates [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ```
        - Declared using square brackets: `[T; N]` where T is type and N is length
        - Zero-based indexing: `array[0]`, `array[1]`, etc.
        - Stack-allocated with known size at compile time
        - Can use repetition syntax: `[initial_value; length]`

## Differences between Tuple and Array:
 - Tuples can store different types, arrays must be same type
 - Tuples have fixed length, cannot grow or shrink
 - Arrays are indexed numerically, tuples use dot notation
 - Arrays are better for homogeneous data, tuples for heterogeneous

## Variables and Mutability:
- Variables are immutable by default in Rust
- Use `mut` keyword to make variables mutable:
```rust
    let x = 5; // immutable
    let mut y = 10; // mutable
    y = 15; // ok because mutable
```
                        
- Default values must be initialized
 - Type inference works for most cases
    
## Borrowing in Rust:
            
- Core concept for memory safety
- Two types of borrows:
- Shared borrow (`&T`): Multiple read-only references
- Mutable borrow (`&mut T`): Single mutable reference
```rust
    let s = String::from("hello");
    let r1 = &s;    // shared borrow
    let r2 = &s;    // multiple shared borrows ok
    let mut s2 = String::from("world");
    let r3 = &mut s2; // single mutable borrow
```
- Cannot have mutable and immutable borrows simultaneously
- Prevents data races at compile time

### Difference between Shared borrow &T and Mutable borrow &mut T:

| Shared borrow (&T)                | Mutable borrow (&mut T)           |
|----------------------------------|-----------------------------------|
| Multiple allowed simultaneously   | Only one allowed at a time        |
| Read-only access                 | Read and write access             |
| Cannot modify data               | Can modify data                   |
| Denoted by &                     | Denoted by &mut                   |
| Safe for concurrent access       | Exclusive access required         |


### Day 2 Checklist – Rust: Data Types & Variables

 **Core Concepts**
- [x] Understood scalar vs compound data types
- [x] Learned about static typing and type inference

**Scalar Types**
- [x] Used signed and unsigned integers
- [x] Used floating-point numbers (`f32`, `f64`)
- [x] Used booleans (`bool`)
- [x] Used characters (`char`)

**Compound Types**
- [x] Created and accessed tuples
- [x] Created and indexed arrays
- [x] Compared tuples and arrays

**Variables and Mutability**
- [x] Declared immutable and mutable variables
- [x] Practiced with `let` and `mut`

**Borrowing**
- [x] Used shared borrow (`&T`)
- [x] Used mutable borrow (`&mut T`)
- [x] Understood borrowing rules and restrictions
