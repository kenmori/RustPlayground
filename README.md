#see README.md in 201811 folder

# RustPlayground

[document](https://doc.rust-lang.org/stable/book/2018-edition/ch01-01-installation.html)

#download
curl https://sh.rustup.rs -sSf | sh

# update

rustup update

# comfirm

rustc --version

# getting start

cd hello_world

# compile

rustc main.rs

# run

./main

# Cargo

Cargo は Rust のビルドシステムでパッケージマネージャー
ほとんどの Rust プログラマーはこのツールを使ってプロジェクト管理をしています
コードの構築、コードが依存するライブラリのダウンロード、それらのライブラリの構築、
等多くのタスクを処理します
先ほどのコードは Cargo の一部部分を使用したが、もっと複雑になると依存関係が追加され、Cargo を追加した方が
はるかに解決しやすくなります

## check

```
cargo --version
```

### Creating a Project with Cargo

cargo でプロジェクトを作る

```
cargo new hello_cargo
cd hello_cargo
```

これで作られたファイルは

Cargo.toml
src/main.rc

### build

cargo を使って build する

```
cargo build
```

実行できるファイルが生成されるので

```
./target/debug/hello_cargo
```

で実行すると

```
Hello, world!
```

cargo run
で一回でそれらをやることができる

cargo check
実行ファイルを生成しないビルドが走る
その分速いのでコードを書いて、開発過程で使うと便利
実行はしない

多くの開発者は
コンパイルできるか check してから cargo build する

ビルドの結果は
target/debug/
にある

# WebAssembly について

Webassembly とはなんですか？
・web のバイナリ実行形式
・汎用の仮想アーキテクチャ
・コンパイルターゲットは c, cpp, rs。コンパイルして.wasm を生成する

それはつまり何？
・JavaScript の置き換え
・プログラミング言語
・全ての動的言語に適したターゲット

Webassembly を選択する理由
直接的なメモリへのアクセス

なぜ asm.js を使わないのですか
いいところ js だけ、速い
悪いところ 非公式仕様 早さに支払えない 拡張のしにくさ 64bit 整数

どうして WebAssembly なの？
asm.js より小さい
より解析が速い
拡張する自由
公式仕様

使い所は ?
GameEngine
マルチメディア 画像/動画編集, 画像認識, ライブビデオ, CAD アプリ
パフォーマンス プラットフォームシミレーション, パスワード保存, 圧縮,暗号化
ライブラリ OpenCV(コンピュータビジョン向けライブラリ。インテルが作成), Box2D(ゲームエンジン), LibSass, DICOM(医療用画像フォーマット)
64-bit match (MAME, SHA512, 医療計算)

どのように使うの？
コンパイラーを使う

オープンソースである LLVM-based で C と C++から JS へ
オープンソースである LLVM-based で C と C++から asm.js へ
オープンソースである LLVM-based で C と C++から WebAssembly へ

Wasm
バイナリ再表現

Wast
テキスト再表現

JavaScriptAPI
fetch(‘half.wasm’)
.then(data => data.arrayBuffer())
.then(buf => WebAssembly.compile(buf))
.then(mod => WebAssembly.instantiate(mod))
.then(ins => alert(ins.exports.half(128)))

何しているの
C や C++で書いたコードをバイナリデータに変換、コンパイルしたのちインスタンスを作りそのメソッドを実行

上と同じことが
Rust でもできる

WebAssembly の未来は？
複数スレッド
SIMD(single instruction multiple data)命令は一つだがそれを複数のデータに適用する。並列化
テスト機能
GC/DOM/WebAPI 統合

If you're a Native developer, (あなたがネイティブの開発者なら)
the Web is just a compiler target away(web はコンパイラのターゲットに過ぎない)

If you're a Web developer, (あなたがウェブの開発者ならば)
you can leverage the enormous world of native libraries.(あなたはネイティブライブラリの巨大な世界を活用できる)

Both worlds have to learn from each other to make the most of this.(両方の世界はこれを最大限に活用するために学ばなければならない)

参照

https://www.hellorust.com/codemotion-ams/slides/

WebAssembly
http://webassembly.org/

WebAssembly info
https://rsms.me/wasm-intro

Rust を使って遊ぶ
https://www.hellorust.com/

RustBook
https://doc.rust-lang.org/book/first-edition/README.html

---wip--
rust をコンパイルするのが Emscripten

Emscripten はデフォルトで形式 asm.js を生成する
asmjs は多くの場面でネイティブに近い速度で実行できる JavaScript のサブセットを高い最適化コードで

Emscripten コードもまた同じくらいのネイティブコードに縮小される

Emscripten Compiler Frontend(emcc)
の setting が.emscripten ファイル
で
Emscripten SDK Manager(emsdk)

Emcc は Clang を使用して C/C++ファイルを LLVM ビットコードに変換し、
Fastcomp(Emscripten のコンパイラコア/LLVM のバックエンド)を使用してビットコードを JavaScript に変換します
出力された JS は node.js やブラウザの html の中で実行できます

Emscripten SDK は複数の SDK とツールを管理するために使われ、
現在のコードをコンパイルするために使用されている特定の SDK/ツールセットを指定するために使用されます
Github から最新のツールチェーンをインストール(ダウンロードしてビルド)することもできます
