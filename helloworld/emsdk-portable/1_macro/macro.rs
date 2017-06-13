macro_rules! say_hello {
    () => (
        println!("here is Macro");
    )
}

fn main(){
    say_hello!()
}