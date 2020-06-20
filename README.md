# 【WIP】Rust練習問題集(Rust practice questions)

<img src="https://kenjimorita.jp/wp-content/uploads/2017/01/ブログrust.png" width="500">

[Author](https://kenjimorita.jp)

 **読み進めるにあたって**

```
- 問題文はfn main{}の中のコンテキストとする
- 未使用のエラーを避けるためplaygroundではprintln!で出力しています
```

**Q1**

Declare the name and initialize it as a string (Hello world)

(nameを宣言して文字列(`Hello world`)として初期化してください)

**A1**
```rust
let name = "Hello world"
```
[playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=1ddad4761679165cc347d60ac455d36d)


**Q2**

Create the `add` function that takes x and y arguments of type i32, adds them and returns them, passes 1 and 2 to the function's arguments from main, executes it and println.

(i32型の引数xとyをとって加算して返す`add`関数を作り、mainからその関数の引数に1と2を渡し、実行し、printlnしてください)

**A2**

```rust
fn add (x:i32, y:i32) -> i32 {
    println!("{}, {}", x, y);
    x + y
} 
fn main(){
    let result = add(1, 2);
    println!("{}", result)
}
```

[playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e2c9cba59d3c08926378ad2bcf9f62a6)
