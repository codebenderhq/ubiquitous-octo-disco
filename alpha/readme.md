# How to run 

and compile/run it with:

```sh
$ rustup target add wasm32-wasi
$ rustc hello.rs --target wasm32-wasi
$ wasmtime hello.wasm
Hello, world!
```

```sh
$ rustup target add wasm32-wasi
$ rustc hello.rs --target wasm32-wasi
$ wasmer hello.wasm
Hello, world!
```
 