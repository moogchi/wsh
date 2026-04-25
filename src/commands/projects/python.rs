#![allow(dead_code)]

use super::common;
use super::default_files;
use crate::syscall;
use crate::syscall::nums::*;

pub fn setup(parts: &[&str], version: &str, name: &str) {
    let std_version = if version.is_empty() { "3.10" } else { version };
    let default_filename = b"main.py\0".to_vec();
    let default_file: &str = default_files::DEFAULT_PYTHON_FILE;
    let python_run = format!("python3");
    let venv_args = ["-m", "venv", ".venv", "--prompt", name];

    common::write_wshproject("python", std_version, name);
    common::run_command(&python_run, &venv_args);

    if syscall::stat(default_filename.as_ptr()) == 0 {
        syscall::print("main.py already exists, skipping\n");
    } else {
        let fd = syscall::open(
            default_filename.as_ptr(),
            O_WRONLY | O_CREAT | O_TRUNC,
            0o755,
        );

        if fd < 0 {
            syscall::print("failed to create main.py\n");
            return;
        }

        syscall::write(fd as i32, default_file.as_bytes());
        syscall::close(fd as i32);
    }

    syscall::print("python project has been created\n");
}

pub fn run(content: &str, parts: &[&str]) {
    let version = content
        .lines()
        .find(|l| l.starts_with("version"))
        .and_then(|l| l.split('=').nth(1))
        .map(|s| s.trim())
        .unwrap_or("3.10");

    let input = if parts.len() >= 2 {
        parts[1]
    } else {
        "main.py"
    };

    let status = common::run_command(".venv/bin/python3", &[input]);

    if status != 0 {
        syscall::print("program not ran\n");
        return;
    }
}
