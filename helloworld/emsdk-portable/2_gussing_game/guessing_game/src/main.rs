use std::io;//userの入力を取得して結果を出すため。ioライブラリのimport
//defaultで読み込むライブラリは「プレリュード」という。
//プレリュードは少しだけなので、なければ直接useする


fn main() {//エントリーポイント。返り値の型がないので空のダブルとして扱われる

    println!("Guess the number");
    println!("please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        println!("You guessed: {}", guess);

}

//mutの意味
//let foo = s; 束縛はイミュータブル
//let mut guess = 5; ミュータブル
//ミュータブルな束縛guessを導入する

//::new()
//::「関連関数」・・スタティックメソッド
//String::new()・・・新たな空の関数を作る。新たな値を作る際に用いられる


//io::stdin()
//https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/std/io/struct.Stdin.html
//関連関数を呼び出している
//もしuse std::io していないならstd::io::stdin()と書くことになる
//ターミナルの標準入力へのハンドルを返す

//read_line
//ハンドルに対してread_lineメソッドを呼んでいる。
//ユーザーからの入力を取得

//expect
//read_lineは値を返す。汎用ライブラリのResult
//Resultのメソッドがexpect
//https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/std/result/enum.Result.html

//よってexpectは返り値でエラーを扱う。実装しないと警告が出る


//{}プレースホルダとして引数にguessを渡している

//cargo run
//if you input 10,
//10