// syscall/mod.rs
#![allow(dead_code)]
#![allow(unused_imports)]
// use macro
#[macro_use]
pub mod macros;

pub mod fd;
pub mod fs;
pub mod net;
pub mod nums;
pub mod proc;
pub mod structs;

pub use fd::*;
pub use fs::*;
pub use net::*;
pub use nums::*;
pub use proc::*;
