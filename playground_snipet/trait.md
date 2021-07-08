# trait

[playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=88cf8e97116de5e6d81d113a4c7b7fc3)

```rust
fn main(){
    let dog = Dog{};
    let cat = Cat{};
    show_animal_data(dog);
    show_animal_data(cat);
}

trait Animal {
    fn lifespan(&self) -> u32;
    fn scientific_name(&self) -> String;
}

struct Dog;
impl Animal for Dog {
    fn lifespan(&self) -> u32 {
        13
    }
    fn scientific_name(&self) -> String {
        "Canis lupus familiaris".to_string()
    }
}

struct Cat;

impl Animal for Cat {
    fn lifespan(&self) -> u32 {
        16
    }
    fn scientific_name(&self) -> String {
        "Felis catus".to_string()
    }
}

fn show_animal_data<T: Animal>(animal: T){
    println!("Lifespan: {} years", animal.lifespan());
    println!("Scientific name: {}", animal.scientific_name());
}
```
