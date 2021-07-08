# 【WIP】Rust練習問題集(Rust practice questions)

<img src="https://kenjimorita.jp/wp-content/uploads/2017/01/ブログrust.png" width="500">

[Author](https://kenjimorita.jp)

 **読み進めるにあたって**

```txt
- 問題文はfn main{}の中のコンテキストとする
- 「出力してください」とはprintln!で出力してくださいの意味です
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

**Q3**

こちら

```rust
fn main() {
    let x = String::from("Hello");
    let y = x;
    println!("x is {}", x);
    println!("y is {}", y);
}
```

はエラーが起きます。正しくしてください

**A3**

```rust
fn main() {
    let x = String::from("Hello");
    let y = &x;
    println!("x is {}", x);
    println!("y is {}", y);
}
```

**Q4**

```rust
fn main() {
    let a = { 10 + 20; };
    println!("a is {}", a);
}
```

はエラーが起きます。正しく治して、原因を教えてください

**A4**

ブロック内にセミコロンを入れると文になります。
()と言う値が変数aに入ってしまう
このaはprintln!マクロで表示ができないのでコンパイルエラーになります

関数を式として扱った場合

```rust
fn main() {
   // 関数は式になる
    let a = add(10, 20);
    println!("a is {}", a);
}
```

**Q5**
for文を使って1から10まで加算し、合計が代入されているsumを表示してください

**A5**
```rust
fn main() {
    let mut sum = 0;
    for i in 0..10 {
        sum += i ;
    }
    println!("sum is {}", sum);
    //sum is 45
}
```

**Q6**
このエラーを解決してください

```rust
error[E0384]: cannot assign twice to immutable variable `sum`
 --> src/main.rs:4:9
  |
2 |     let sum = 0;
  |         ---
  |         |
  |         first assignment to `sum`
  |         help: make this binding mutable: `mut sum`
3 |     for i in 0..10 {
4 |         sum += i;
  |         ^^^^^^^^ cannot assign twice to immutable variable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0384`.
error: could not compile `hello`.
```

**A6**

```rust
fn main() {
    let sum = 0;
    for i in 0..10 {
        sum += i;
    }
    println!("sum is {}", sum);
}
```


**Q7**
ビット演算子とはなんですか？

**A7**
コンピューターの0と1のビット単位を計算するための特別な演算子
- CPUは主に8ビット(1バイト)の単位で計算を行う(0と1の8桁でで表現すること8ビットを1バイトとする)


**Q8**
”hello”をprintするだけの関数no_paramをmain関数の中で呼び出してください。

**A8**

```rust
fn main() {
    no_param();
}

fn no_param () {
    println!("hello");
}
```

**Q9**
1と2を引数x, yとして渡してそれらを出力する関数を定義して実行してください

**A9**

```rust
fn main() {
    tow_param(1, 2);
}

fn tow_param (x:i32, y:i32) {
    println!("x is {}, y is {}", x, y);
}
```

**Q10**
1と2を引数x, yとして渡し、それを合計した数を返す関数を定義して、実行、受け取った値を出力してください

```rust
fn main() {
    let xy = add_param(1, 2);
    println!("xy is {}", xy)
}

fn add_param (x:i32, y:i32) -> i32 {
    println!("x is {}, y is {}", x, y);
    x + y
}
```

**Q11**
i32型のxフィールドを持つSampleを作り、
Sampleが持つメソッド、int(xに1を加算した値を返す)、add(引数を取り、xに引数と加算した値を返す)の2つ定義して呼び出してください

**A11**

- main関数の中でコンストラクタでxを初期化し、
- int関数xを引数にとり、1を足したi32型の値を返す
- add関数 xを引数にとりxフィールドにxを足した値を返す
- 呼び出し時の値はなんでもよい

```rust
struct Sample {
    x: i32
}
impl Sample {
    fn new (x: i32) -> Sample {
        Sample{x: x}
    }
    fn int(&self) -> i32 {
        self.x + 1
    }
    fn add(&self, x: i32) -> i32 {
        self.x + x
    }
}

fn main() {
    let a = Sample::new(1);
    let ans = a.int();
    println!("ans is {}", ans);
    let ans2 = a.add(30);
    println!("ans2 is {}", ans2);

}
```

**Q12**
xを引数にとって、mainブロック内に定義されたi32型のnumと加算するクロージャー(関数内で定義された関数)を実行して出力してください
**A12**

```rust
fn main() {
    let num = 1;
    let add = |x|{num + x};
    let ans = add(2);
    println!("x is {}", ans);
}
```

**Q13**
こちらはコンパイルエラーになります

```rust
fn main() {
    let x = String::from("a");
    let y = x;
    println!("x is {}, y is {}", x, y);
}
```

xの値をyに束縛、その後xを表示しようとしています。
何故だか教えてください。また正しく書いてください

**A13**
xの所有権はyに移っているためxは参照するものがありません。
もしxの参照先を渡したい場合
&xとします

```rust
fn main() {
    let x = String::from("a");
    let y = &x;
    println!("x is {}, y is {}", x, y);
}
```

こちら

```rust
fn main() {
    let x = String::from("a”); // ここは_xとする必要がありますが問題の伝え易さのためそうしています
    let x = String::from("b");
    println!("x is {}", x);
}
```

のように同じ変数名で定義し直すことをなんと言いますか。

**A14**
シャドーイング
後から値を変更できる
最初のlet xの定義を覆い隠すシャドーイングすることができる
最初のxと次のxは別物
束縛する方は数値でも文字列でも可能

**Q15**
こちらはコンパイルエラーになります

```rust
fn main() {
    let x = String::from("a”);
    println!("x is {}", x);
    x = String::from("b");
    println!("x is {}", x);
}
```

何故ですか、またxが再束縛なように修正してください

**A15**
Rustではletで宣言された変数は再束縛できません。(変数なのに)
後から再束縛する必要がある場合、宣言時mutをつける必要があります

```rust
fn main() {
    let mut x = String::from("a");
    println!("x is {}", x);
    x = String::from("b");
    println!("x is {}", x);
}
```


**Q16**

こちらの関数が何も返さない時の型を明示的に書いてください

```rust
fn foo(_x: &'static str) {
}
```

**A16**

```rust
fn foo(_x: &'static str) ->() {
}

fn main() {
    foo("bar")
}
```

**Q18**

こちらと同じ意味のsliceを書いてください

```rust
let s = String::from("hello");

let slice = &s[0..2]; // here
```

**A18**

```rust
let slice = &s[..2];


// additional
// 1, 2 is equal
let s = String::from("hello");
let len = s.len();

let slice = &s[3..len]; //1
let slice = &s[3..]; //2

// 3,4 is equal
let slice = &s[0..len]; //3
let slice = &s[..]; //4

```

**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```

**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```

**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```


**Q17**

```rust
```

**A17**

```rust
```

**Q17**

```rust
```

**A17**

```rust
```



参照

[プログラミング言語Rust入門 Kindle版](https://www.amazon.co.jp/%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E8%A8%80%E8%AA%9ERust%E5%85%A5%E9%96%80-%E5%A2%97%E7%94%B0-%E6%99%BA%E6%98%8E-ebook/dp/B087BZQ48R/ref=pd_sbs_351_2/355-5873983-9317719?_encoding=UTF8&pd_rd_i=B087BZQ48R&pd_rd_r=1634473b-dcef-4b41-b894-18e716747249&pd_rd_w=CooXQ&pd_rd_wg=7kSK2&pf_rd_p=7642417c-6494-4d06-a2b0-fcb0e0b3c563&pf_rd_r=931KKDVY2WCJYNY84P7E&psc=1&refRID=931KKDVY2WCJYNY84P7E)