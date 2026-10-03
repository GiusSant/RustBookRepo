use std::{dbg, println};

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 30
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    println!("rect1 is {rect1:#?}");

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };
    dbg!(&rect2);

    println!(
        "the area of the rectangle with the method is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle {
        width: 10,
        height:10
    };

    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(5);
    println!("{rect4:#?}");

    let r = &mut Box::new(Rectangle { 
        width: 1,
        height: 2
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area_struct (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}