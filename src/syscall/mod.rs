// syscall/mod.rs
#![allow(dead_code)]
#![allow(unused_imports)]
// use macro
#[macro_use]
pub mod macros;

pub mod fs;
pub mod net;
pub mod nums;
pub mod proc;

pub use fs::*;
pub use net::*;
pub use nums::*;
