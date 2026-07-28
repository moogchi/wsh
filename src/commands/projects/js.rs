#![allow(dead_code)]

use super::common;
use super::default_files;
use crate::syscall;
use crate::syscall::nums::*;

pub fn setup(_parts: &[&str], _version: &str, name: &str) {
    let default_filename = b"main.js\0".to_vec();
    let default_file: &str = default_files::DEFAULT_JS_FILE;

    common::write_wshproject("js", "latest", name);

    if syscall::stat(default_filename.as_ptr()) == 0 {
        syscall::print("main.js already exists, skipping\n");
    } else {
        let fd = syscall::open(
            default_filename.as_ptr(),
            O_WRONLY | O_CREAT | O_TRUNC,
            0o644,
        );

        if fd < 0 {
            syscall::print("failed to create main.js\n");
            return;
        }

        syscall::write(fd as i32, default_file.as_bytes());
        syscall::close(fd as i32);
    }

    syscall::print("js project has been created\n");
}

pub fn run(_content: &str, parts: &[&str]) {
    let input = if parts.len() >= 2 {
        parts[1]
    } else {
        "main.js"
    };

    let status = common::run_command("node", &[input]);

    if status != 0 {
        syscall::print("program not ran\n");
    }
}
