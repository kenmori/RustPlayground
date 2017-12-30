macro_rules! say_hello {
    () => (
        println!("here was Macro");
    )
}

fn main(){
    say_hello!()
}