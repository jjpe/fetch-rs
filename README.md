# fetch-rs

![](https://img.shields.io/badge/rustc-1.46+-darkcyan.svg)

This crate is a Foreign Function Interface (FFI) that allows Rust
code to call the JavaScript `fetch` API when deployed on WASM.

The main differences between this crate and using the `web_sys`-provided
version of the `fetch` API directly are:
1. The `web_sys`-provided version only works in the browser. It
   will compile when targeting NodeJS, but will fail at runtime.
2. Ergonomics. The `web_sys` version is not very rusty, while
   this crate aims to be.


## Usage

**NOTE** Keep in mind that:
* This crate is _only_ usable on WASM. This means that any crate that uses
  `fetch-rs` will stop compiling to native code.
* Because the `fetch` API is inherently `async`, so is this API. This property
  is infectious, meaning that any code using this crate would to well to convert
  to async as well.
* There are limitations that are imposed by virtue of depending on the
  [node-fetch](https://www.npmjs.com/package/node-fetch) NPM package.
  See the NPM page for more details on that.

### preparation


If you aim to test code using e.g. NodeJS or the browser, install
`wasm-pack` by executing `cargo install wasm-pack`.

### Cargo.toml

Add this to your Cargo.toml:
``` toml
[dependencies]
fetch-rs = "0.1"
```

### Create and send request

1. `GET`-request:
``` rust
use fetch_rs::Request;

#[wasm_bindgen] // <-- This makes the fn callable from JavaScript for easy testing
pub async fn perform_get_request(url: String) -> JsValue {
    let result: JsValue = Request::get(&url)
        .fetch_text() // fetch the result as text
        .await;
    info!("[test] result: {:?}", result); // Write test output
    result // Since the fn is async, return a Promise / Future
}
```

2. `POST`-request:
``` rust
use fetch_rs::Request;

#[wasm_bindgen] // <-- This makes the fn callable from JavaScript for easy testing
pub async fn perform_post_request(url: String) -> JsValue {
    let result: JsValue = Request::post(&url)
    .fetch_text() // fetch the result as text
    .await;
    info!("[test] result: {:?}", result); // Write test output
    result // Since the fn is async, return a Promise / Future
}
```


For more examples, check out [the examples module](./src/lib.rs#L17)
in the src directory.
