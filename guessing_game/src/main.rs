fn main() {
    println!("Hello, world!");
    println!("{}", is_divisible_by(100, 200))
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}