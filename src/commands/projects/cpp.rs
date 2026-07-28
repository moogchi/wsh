#![allow(dead_code)]

use super::common;
use super::default_files;
use crate::syscall;
use crate::syscall::nums::*;

pub fn setup(_parts: &[&str], version: &str, name: &str) {
    let std_version = if version.is_empty() { "17" } else { version };
    let default_filename = b"main.cpp\0".to_vec();
    let default_file: &str = default_files::DEFAULT_CPP_FILE;

    common::write_wshproject("cpp", std_version, name);

    if syscall::stat(default_filename.as_ptr()) == 0 {
        syscall::print("main.cpp already exists, skipping\n");
    } else {
        let fd = syscall::open(
            default_filename.as_ptr(),
            O_WRONLY | O_CREAT | O_TRUNC,
            0o755,
        );

        if fd < 0 {
            syscall::print("failed to create main.cpp\n");
            return;
        }

        syscall::write(fd as i32, default_file.as_bytes());
        syscall::close(fd as i32);
    }

    syscall::print("cpp project has been created\n");
}

pub fn run(content: &str, parts: &[&str]) {
    let version = content
        .lines()
        .find(|l| l.starts_with("version"))
        .and_then(|l| l.split('=').nth(1))
        .map(|s| s.trim())
        .unwrap_or("17");

    let std_flag = format!("-std=c++{}", version);
    let output = if parts.len() >= 4 && parts[2] == "-o" {
        parts[3].to_string()
    } else {
        "main".to_string()
    };
    let input = if parts.len() >= 2 {
        parts[1]
    } else {
        "main.cpp"
    };

    syscall::print("running: g++ ");
    syscall::print(&std_flag);
    syscall::print(" ");
    syscall::print(input);
    syscall::print(" -o ");
    syscall::print(&output);
    syscall::print("\n");

    let status = common::run_command("g++", &[&std_flag, input, "-o", &output]);

    // check if compile worked
    if status != 0 {
        syscall::print("compile failed\n");
        return;
    }
    // now run
    common::run_command("./main", &["./main"]);
}
