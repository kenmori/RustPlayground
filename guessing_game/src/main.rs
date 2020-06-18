use std::io;

fn main () {
    println!("pleas the number");
    println!("pleas the your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("fail");
    println!("{}", guess);
}
