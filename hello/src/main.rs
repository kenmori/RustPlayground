fn main() {
    let a = String::from("fafa");
    let b = String::from("bbb");
    let c = a + &b;
    println!("{}", c);
}
