//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

wasm_bindgen_test_configure!(run_in_browser);

fn pass() {
    assert_eq!(1 + 1, 2);
}
