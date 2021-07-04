
# async await

## code

[playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=61af524bf7b903d6cc17e9c54070ec4d)


```rs

use futures::executor;

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn something_great_async_function() -> i32 {
    let ans = async_add(1, 2).await;
    let ans2 = async_add(3, 5).await;
    println!("{}", ans + ans2);
    ans
}
fn main() {
    println!("Hello, world!");
    executor::block_on(something_great_async_function());
}

```
