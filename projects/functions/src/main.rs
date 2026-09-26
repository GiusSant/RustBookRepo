fn main() {
    println!("Hello, world!");

    another_function();

    another_function_2(5);

    print_labeled_measurements(5, String::from("cm"));

    let x = five();
    println!("The value of x is {x}");

    let y = plus_one(x);
    println!("The value of y is {y}");
}

fn another_function() {
    println!("Another function");
}

fn another_function_2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurements(value: i32, unit_label: String) {
    println!("The measurement is {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}