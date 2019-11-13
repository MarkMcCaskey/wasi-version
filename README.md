# WASI Version

A tiny command line utility for identifying a Wasm module's [WASI version][wasi-version].

Install it with

```sh
cargo install wasi-version
```

or run it via Wasm by installing [Wasmer] and downloading it from [wapm]:

```sh
wapm install -g mark/wasi-version
```

Note, that when running it via Wasm, you must pass `--dir=.` with an argument of `.` or any other directory from which the Wasm module can be reached.

[wasi-version]: https://github.com/WebAssembly/WASI/tree/master/phases
[Wasmer]: https://wasmer.io/
[wapm]: https://www.wapm.io/
