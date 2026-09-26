use std::println;
use std::io;

fn main() {
    let a: u8 = 3;
    let b: i8 = -3;

    let c = 2.0;
    let d: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 3;

    let t = true;
    let f: bool = false;

    let e = 'z';
    let f: char = 'Z';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {y}");
    let five_hundred = tup.0;
    let six_points_four = tup.1;
    let one = tup.2;

    let mut tup1 = (1,2);
    tup1.0 = 0;
    tup1.1 += 5;

    let arr = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let arr2 = [3;5];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
