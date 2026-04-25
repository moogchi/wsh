#![allow(dead_code)]

use super::common;
use super::default_files;
use crate::syscall;
use crate::syscall::nums::*;

pub fn setup(_parts: &[&str], _version: &str, name: &str) {
    // use cargo
    common::write_wshproject("rust", "check --version", name);
    let init_status = common::run_command("cargo", &["init", "."]);

    if init_status != 0 {
        syscall::print("cargo init failed\n");
        return;
    }

    let default_filename = b"src/main.rs\0".to_vec();
    let default_file: &str = default_files::DEFAULT_RUST_FILE;

    let fd = syscall::open(
        default_filename.as_ptr(),
        O_WRONLY | O_CREAT | O_TRUNC,
        0o644,
    );

    if fd < 0 {
        syscall::print("failed to create src/main.rs\n");
        return;
    }

    syscall::write(fd as i32, default_file.as_bytes());
    syscall::close(fd as i32);

    syscall::print("rust project has been created\n");
}

pub fn run(content: &str, parts: &[&str]) {
    let fmt_status = common::run_command("cargo", &["fmt"]);

    //check fmt
    if fmt_status != 0 {
        syscall::print("fmt failed");
        return;
    }

    let check_status = common::run_command("cargo", &["check"]);

    //check
    if check_status != 0 {
        syscall::print("check failed");
        return;
    }

    // now run
    common::run_command("cargo", &["run"]);
}
