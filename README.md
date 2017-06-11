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

