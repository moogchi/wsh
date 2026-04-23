// commands/mod.rs
pub mod builtins;
pub mod projects;
pub mod socket;

use crate::syscall;
use crate::syscall::fs::*;
use crate::syscall::nums::*;
use crate::syscall::proc::*;

pub fn run(cmd: &str) {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "cd" | "pwd" | "echo" | "exit" => builtins::run(&parts),
        "opensocket" | "accept" | "send" | "respond" | "closesocket" => socket::run(&parts),
        "setproject" | "openproject" | "run" => projects::run(&parts),
        _ => builtins::run(&parts),
    }
}
