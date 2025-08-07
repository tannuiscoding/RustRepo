# 📚 Index – Day 2: Data Types & Variables in Rust

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
     - Accessing values via indexing
   - Arrays
     - Fixed-size and same-type
     - Accessing values
     - Repetition syntax

4. **Tuple vs Array Comparison**
   - Differences in use cases, typing, and syntax

5. **String**  
   - String types: `String` vs `&str`
   - Option and Result types
   - Variables: `let`, `mut`, shadowing



## Data types in rust:

1. Scalar Data types: they hold single values.
    - Integer (both signed and unsigned): they are denotes by i8 to i28 for signed integers and from u8 to u128 for unsigneed integers. 
        - the numbers from i8 to i128 grow in power of 2. like i8, i16... so on. 
        - the number in the sign represent the number of bits. 
        - i[n] means - 2^(n-1) -1 to 2^(n-1) -1. 
    - floating point (for float values): 
      - `f32`: 32-bit
      - `f64`: 64-bit *(default)*
    - Boolean:
       - `bool`: `true` or `false`
    - Character:
       - `char`: A Unicode scalar (e.g., `'a'`, `'中'`, `'∞'`)

2. Compound data types: it can group multiple values into one type. Rust has two built in compound data types.
    - tuples:
        - A tuple is a fixed-size collection that can hold values of different types.
        - it is of fixed length.
        - You can access values by dot notation with zero-based indexing (.0, .1, etc.).

    - Array (like every other language): 

