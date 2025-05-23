#[derive(Debug)]

struct Rectangle {
    width: u64,
    height: u64,
}

// Methods are functions that are associated with a struct.
impl Rectangle {
    fn area(&self) -> u64 {
        self.width * self.height
    }

    fn can_fit(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Associated functions are functions that are associated with a struct, but they are not methods.
impl Rectangle {
    fn new(width: u64, height: u64) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: u64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 10,
        height: 20,
    };

    let r1 = Rectangle {
        width: 5,
        height: 10,
    };

    let r2 = Rectangle {
        width: 15,
        height: 25,
    };

    let r3 = Rectangle::new(30, 40);
    let r4 = Rectangle::square(50);

    // println!("can fit r1 in Rectangle: {}", rectangle.can_fit(&r1));
    // println!("can fit r2 in Rectangle: {}", rectangle.can_fit(&r2));
    // println!("The area of the rectangle is: {}", rectangle.area());

    println!("The area of the r1 is: {:?}", r1);
    println!("The area of the r2 is: {:?}", r2);
    println!("The area of the r3 is: {:?}", r3);
    println!("The area of the r4 is: {:?}", r4);
}
