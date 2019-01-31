
fn main(){
 println!("{}, {}", "hell", "world");
 println!("{0},{1}!", "Hello","world");
 println!("{greeting}, {name}!", greeting="Hell", name="world");
 println!("{:?}", [1,2,3]);
 println!("{:#?}",[1,2,3]);
 let x = format!("{},{}!", "hell", "world");
 println!("{}", x);
}