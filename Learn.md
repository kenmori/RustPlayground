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


