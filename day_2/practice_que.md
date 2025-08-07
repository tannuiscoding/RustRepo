# Practice Questions – Day 2: Data Types & Variables in Rust

## Scalar Types
1. Declare a signed integer, an unsigned integer, and a floating-point number. Print them.

```rust
fn main() {
   let int_8: i8 = 127;
   let float_32: f32 = 3.14;
   println!("int_8: {}", int_8); //{} is used to print the value of the variable. it is a format specifier
   println!("float_32: {float_32}"); //{float_32} this can also be used
}
```
2. Create a variable to store whether a number is even using a boolean.

```rust
fn main() {
   let mut num = String::new();
   println!("Enter a number: ");
   io::stdin().read_line(&mut num).expect("Failed to read");
   let num: i32 = num.trim().parse().expect("Please enter a valid number");

   let is_even: bool = even_odd(num);
   if is_even{
      println!("The number is even");
   }else{
      println!("The number is odd");
   }
}

fn even_odd(num : i32) -> bool{
   if num % 2 == 0{
      return true;
   }else{
      return false;
   }
}
```

3. Store and print a Unicode character like '₹' or an emoji using `char`.

```rust
fn main() {
   let char_1: char = '₹';
   let char_2: char = '😀';
   println!("char_1: {}", char_1);
   println!("char_2: {}", char_2);
}
```

4. What will happen if you try to assign a negative number to a `u8` variable?

```rust
fn main() {
   let num: u8 = -1;
   println!("num: {}", num); //it will give an error because u8 can only store positive numbers.
}
```
this is the error that will be shown:
```rust
error[E0600]: cannot apply unary operator `-` to type `u8`
```

## Compound Types
5. Create a tuple with values of different types. Destructure it and print each value.

```rust
fn main(){
    let tuple = (1, 2.0, 'a', "tannu");
    let (a, b, c, d) = tuple;
    println!("tuple: {:?}", tuple); //{:?} is used to print the value of the tuple.
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
}
```

6. Access the second value of a tuple using dot notation.
7. Create an array of 5 integers and print the first and last elements.
8. Use repetition syntax to create an array of ten zeroes.
9. What happens if you try to access an index that doesn't exist in the array?

## Tuple vs Array
10. Explain in code the difference between an array and a tuple.
11. When would you prefer using a tuple over an array? Write a small example.

## Variables and Mutability
12. Declare a mutable variable and change its value.
13. Try reassigning a value to an immutable variable and observe the error.
14. Create a variable using type inference and another with an explicit type.

## Borrowing
15. Create a function that takes a shared reference to a string and prints it.
16. Create a function that takes a mutable reference and changes the value.
17. Try creating both a mutable and immutable borrow in the same scope. What error do you get?
18. Explain in code how Rust prevents data races at compile time.

## Bonus
19. Write a program that calculates the square of a number using a function and prints the result.
20. Create a function that returns a tuple containing the sum and product of two numbers.
