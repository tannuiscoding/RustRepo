//Solution for First Question

// fn main() {
//    let int_8: i8 = 127;
//    let float_32: f32 = 3.14;
//    println!("int_8: {}", int_8);
//    println!("float_32: {float_32}");
// }

//Solution for Second Question

// use std::io;
// fn main() {
//    let mut num = String::new();
//    println!("Enter a number: ");
//    io::stdin().read_line(&mut num).expect("Failed to read");
//    let num: i32 = num.trim().parse().expect("Please enter a valid number");

//    let is_even: bool = even_odd(num);
//    if is_even{
//       println!("The number is even");
//    }else{
//       println!("The number is odd");
//    }
// }

// fn even_odd(num : i32) -> bool{
//    if num % 2 == 0{
//       return true;
//    }else{
//       return false;
//    }
// }

//Solution for Third Question

// fn main() {
//     let char_1: char = '₹';
//     let char_2: char = '😀';
//     println!("char_1: {}", char_1);
//     println!("char_2: {}", char_2);
//  }

//Solution for Fourth Question

// fn main() {
//     let num: u8 = -1;
//     println!("num: {}", num);
//  }

fn main(){
    let tuple = (1, 2.0, 'a', "tannu");
    let (a, b, c, d) = tuple;
    println!("tuple: {:?}", tuple); //{:?} is used to print the value of the tuple.
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
}