# rust

- Variables and Datatypes
- Operators
- Conditional Statements
- Loops
- Functions
- Strings
- Vectors
- Structs
- Enums
- Traits
- Generics
- Modules
- Memory Management
- Ownership
- Borrowing
- Lifetime

## printlin

### Single Placeholder

### Multiple Placeholders

### Positional Arguments

```rust
fn main() {
    println!("Enhance your coding skills from {0} courses.  {0} courses are very {1}", "Educative", "interactive");
}
```

### Named Arguments

```rust
fn main() {
    println!("{company} provides {kind} courses", company = "Educative", kind = "interactive");
}
```

### Placeholder Traits

```rust
{:b},{:x},{:o}
```

### Basic Math

```rust
fn main() {
    println!("{} + {} = {}",10, 10, 10 + 10);
}
```

### Placeholder for a Debug Trait

複数を一辺に出力できる

```rust
fn main() {
    println!("{:?}", ("This is a Rust Course", 101));
}
```

## Print System

### print!()

one line

### println!()

### eprint!()

### eprintln!()


## Types of Comments in Rust

### Line Comments //

### Block Comments /…/

### Doc Comments

### Outer Doc Comments ///
codeの外でmarkdownで書ける


### Inner Doc Comments //!
codeの中で使える


## Variables

### Create a Variable

### Initialize a Variable

### What if You Want to Make a Variable Mutable?

### Assigning Multiple Variables

使わない値だったり、使わないmutはこちらのアトリビュートを書くとよい
`#[allow(unused_variables, unused_mut)]`

## Scope of a variable

### Types of Variables

### Local Variable

### Global Variable

### Shadowing


## type

### Scalar Type

They store a single value.Below is the list of scalar types:

#### Numeric Types: Integers and Floats

- Integers
- Floats

##### Variable Size Types

- i8: The 8-bit signed integer type.
- i16: The 16-bit signed integer type.
- i32: The 32-bit signed integer type.
- i64: The 64-bit signed integer type.
- u8: The 8-bit unsigned integer type.
- u16: The 16-bit unsigned integer type.
- u32: The 32-bit unsigned integer type.
- u64: The 64-bit unsigned integer type.
- isize: The pointer-sized signed integer type.
- usize: The pointer-sized unsigned integer type.

プログラマーは、データが失われるほど小さくないデータ型を選択する必要があります。
また、メモリを浪費するほど大きいデータ型を選択するべきではありません。

##### Floating Point

- f32: The 32-bit floating point type.
- f64: The 64-bit floating point type.

##### Boolean

##### Character

Rustは1文字4bite使用する

### Compound Type

They group multiple values in one variable. Below is the list of compound

- Array
An array is a homogenous sequence of elements.

- Tuple

### const

- `global value`が定義できる(letは定義できない)
- `mut`が使えない。可変ではない
- data型を定義する必要がある
- 実行時ではなくコンパイル時に値が入っていないといけない
- シャドーイングができない


型が違う数値をasで変換して足す

```rust
let c = a as u32 + b;
```

何度も使われるような定数はconst
大文字のスネークケースで

```rust
const PI: f32 = 3.14159; // 型の指定は必要
```


関数に戻り値が指定されていない場合`()`が返される

```rust
fn a (){
    // ()と推論される。()はunitと呼ばれる空のタプル
}
```

以下は同じ意味。 ..= 演算子。`0..=5`はイテレータを作る

```rust
for x in 0..5 {
    println!("{}", x);
}

for x in 0..=5 {
    println!("{}", x);
}
```


```rust
match x {
    0 => {
        println!("found zero");
    }
    // 複数の値にマッチ
    1 | 2 => {
        println!("found 1 or 2!");
    }
    // 範囲にマッチ
    3..=9 => {
        println!("found a number 3 to 9 inclusively");
    }
    // マッチした数字を変数に束縛
    matched_num @ 10..=100 => { // variables@で変数を作る
        println!("found {} number between 10 to 100!", matched_num);
    }
    // どのパターンにもマッチしない場合のデフォルトマッチが必須
    _ => {
        println!("found something else!");
    }
}
```

loopで抜ける時はbreakで抜ける。その際に値を返せる

```rust
let v = loop {
    x += 1;
    if x == 11 {
        break "find 11";
    }
};

```

`;`がないものがreturnされる

```rust
let v = {
    let a = 1;
    let b = 2;
    a + b
};
println!("from block: {}", v);

```

static or instance

```rust
// static method - ある型そのものに紐付き、演算子::で呼び出せる

// instance method - ある型のインスタンスに紐付き、演算子.で呼び出せる
```

3つのメモリ空間

```rust
// データメモリ・・・ 固定長もしくは スタティック (例: プログラムのライフサイクルで常に存在するもの)なデータ

// スタックメモリ・・・関数内で宣言された変数。 関数が呼び出されている間は、メモリ上の位置は変更されることがない

// ヒープメモリ・・・プログラムの実行時に作られるデータ。
// データをヒープメモリに入れることをアロケーション(allocation)といい、
// データをヒープメモリから削除することはディアロケーション(deallocation)と言います。
```

```rust
// ユニットライクな構造体
// Rust ではフィールドを持たない構造体を宣言できます。
struct Marker;
```


```rust
 // Result の Ok の中にある unit 値によって、
    // すべてが正常であることを表現していることに注意してください。
    Ok(())
```


簡潔なエラー処理

```rust
let v = do_something_that_might_fail()?
// or
match do_something_that_might_fail() {
    Ok(v) => v,
    Err(e) => return Err(e),
}
```

unwrap()が何をしているか

```rust
// Option/Result 内の値を取得します。
// 列挙型が None/Err の場合、panic! します。
```


```rust

```
> impl の後ろに <T> を宣言しています．こうすることで Point<T> の T がジェネリック型であることを明示しています．もし， impl<T> でなければ， Point<T> の T はジェネリック型ではなく T という名前の型を指定することになってしまいます


```rust

```


> 同一オブジェクトに対する参照と可変について，いくつか制限

> 不変参照 (&) は何個でも同時に存在することが出来る
> 不変参照 (&) と可変参照 (&mut) は同時に存在することが出来ない。

同時に定義された際に破棄される。その後破棄されたものを参照した場合エラー

> 可変参照 (&mut) は同時に１つしか存在することが出来ない
> 借用チェックは関数呼び出し時(かつコンパイル時)にされる
- 借用チェック時に制約を満たしているかがポイント



```rust

```

matchした際にエラー

from

```rust
match x {
    a => println!("{}", a); // ここでownerが変わっている
}
println!("{}", a); // error
```

to

```rust
match x {
    Some(ref a) => println!("{}", a); // ここでownerが変わっている. // ref mut aとか
}
println!("{}", a); // error
```

```

分解パターンの

```rust
  let a = Account { name: String::from("name"), pass: String::from("pass") };let Account { ref name, ref pass } = a;
```


if letで記述を簡潔に

```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```


マッチガード

```rust
Some(x) if x < 5 => println!("less than five: {}", x),
```


```rust
```


ref:

[Rust入門](https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/8ccf20)
[https://tourofrust.com/chapter_2_ja.html](https://tourofrust.com/chapter_2_ja.html)