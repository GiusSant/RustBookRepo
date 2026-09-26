fn main() {
    //IF
    println!("IF/ELSE");
    let  number = 4;
    if number < 5 {
        println!("condition was true");
    }
    else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    }
    else if number % 3 == 0 {
        println!("number is divisible by 3");
    }
    else if number % 2 == 0 {
        println!("number is divisible by 2");
    }

    let condition = true;
    let number = if condition {5} else {6};
    println!("the valus of number is {number}");
    println!("-------------");

    //LOOPS
    println!("LOOPS");
    let mut counter = 0;
    let result = loop {
        counter+=1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is {result}");
    println!("--------------");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining-=1;
        }
        count+=1;
    }
    println!("end count = {count}");
    println!("--------------");
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number-=1;
    }
    println!("--------------");
    let arr = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("the value of x is: {}", arr[index]);
        index+=1;
    }
    println!("--------------");
    for element in arr {
        println!("the value is: {element}");
    }
    println!("---------------");
    for number in (1..4).rev() {
        println!("{number}");
    }

}
