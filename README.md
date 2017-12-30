# RustPlayground

#download
curl https://sh.rustup.rs -sSf | sh

# update
source $HOME/.cargo/env

# comfirm
rustc --version

# compile
rustc main.rs

# out
./main

--add 2017/06/11
ref https://kripken.github.io/emscripten-site/docs/tools_reference/emsdk.html#emsdk-howto

cd emsdk-portable
create .rs file

#compile
rustc --target=wasm32-unknown-emscripten hello.rs -o hello.html
rustc --target=wasm32-unknown-emscripten hello.rs -o hello.js



WebAssemblyについて



Webassemblyとはなんですか？
・webのバイナリ実行形式
・汎用の仮想アーキテクチャ
・コンパイルターゲットはc, cpp, rs。コンパイルして.wasmを生成する

それはつまり何？
・JavaScriptの置き換え
・プログラミング言語
・全ての動的言語に適したターゲット

Webassemblyを選択する理由
直接的なメモリへのアクセス

なぜasm.jsを使わないのですか
いいところ jsだけ、速い
悪いところ 非公式仕様 早さに支払えない 拡張のしにくさ 64bit整数

どうしてWebAssemblyなの？
asm.jsより小さい
より解析が速い
拡張する自由
公式仕様

使い所は ?
GameEngine
マルチメディア 画像/動画編集, 画像認識, ライブビデオ, CADアプリ
パフォーマンス プラットフォームシミレーション, パスワード保存, 圧縮,暗号化
ライブラリ OpenCV(コンピュータビジョン向けライブラリ。インテルが作成), Box2D(ゲームエンジン), LibSass, DICOM(医療用画像フォーマット)
64-bit match (MAME, SHA512, 医療計算)


どのように使うの？
コンパイラーを使う

オープンソースであるLLVM-basedでCとC++からJSへ
オープンソースであるLLVM-basedでCとC++からasm.jsへ
オープンソースであるLLVM-basedでCとC++からWebAssemblyへ


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
CやC++で書いたコードをバイナリデータに変換、コンパイルしたのちインスタンスを作りそのメソッドを実行

上と同じことが
Rustでもできる

WebAssemblyの未来は？
複数スレッド
SIMD(single instruction multiple data)命令は一つだがそれを複数のデータに適用する。並列化
テスト機能
GC/DOM/WebAPI統合


If you're a Native developer, (あなたがネイティブの開発者なら)
the Web is just a compiler target away(webはコンパイラのターゲットに過ぎない)

If you're a Web developer, (あなたがウェブの開発者ならば)
you can leverage the enormous world of native libraries.(あなたはネイティブライブラリの巨大な世界を活用できる)

Both worlds have to learn from each other to make the most of this.(両方の世界はこれを最大限に活用するために学ばなければならない)

参照

https://www.hellorust.com/codemotion-ams/slides/

WebAssembly
http://webassembly.org/

WebAssembly info
https://rsms.me/wasm-intro

Rustを使って遊ぶ
https://www.hellorust.com/

RustBook
https://doc.rust-lang.org/book/first-edition/README.html




---wip--
rustをコンパイルするのがEmscripten

Emscriptenはデフォルトで形式asm.jsを生成する
asmjsは多くの場面でネイティブに近い速度で実行できるJavaScriptのサブセットを高い最適化コードで


Emscriptenコードもまた同じくらいのネイティブコードに縮小される

Emscripten  Compiler Frontend(emcc)
のsettingが.emscriptenファイル
で
Emscripten SDK Manager(emsdk)

EmccはClangを使用してC/C++ファイルをLLVMビットコードに変換し、
Fastcomp(Emscriptenのコンパイラコア/LLVMのバックエンド)を使用してビットコードをJavaScriptに変換します
出力されたJSはnode.jsやブラウザのhtmlの中で実行できます

Emscripten SDKは複数のSDKとツールを管理するために使われ、
現在のコードをコンパイルするために使用されている特定のSDK/ツールセットを指定するために使用されます
Githubから最新のツールチェーンをインストール(ダウンロードしてビルド)することもできます

