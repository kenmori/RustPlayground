fn main() {
    println!("Hello, world!");
    println!("{}", is_divisible_by(100, 200));
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("Rectangle perimeter: {}", 32);
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

struct Rectangle {
    p1: Point,
    p2: Point,
}
struct Point {
    x: f64,
    y: f64
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.1}
    }

    fn new(x: f64, y: f64) -> Point {
        Point {x: x, y: y}
    }
}