#![cfg_attr(any(target_arch = "wasm32", not(feature = "std")), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;
