#![allow(dead_code)]

use super::common;
use super::default_files;
use crate::syscall;
use crate::syscall::nums::*;

pub fn setup(_parts: &[&str], _version: &str, name: &str) {
	let default_filename = b"Main.java\0".to_vec();
	let default_file: &str = default_files::DEFAULT_JAVA_FILE;

	common::write_wshproject("java", "latest", name);

	if syscall::stat(default_filename.as_ptr()) == 0 {
		syscall::print("Main.java already exists, skipping\n");
	} else {
		let fd = syscall::open(
			default_filename.as_ptr(),
			O_WRONLY | O_CREAT | O_TRUNC,
			0o644,
		);

		if fd < 0 {
			syscall::print("failed to create Main.java\n");
			return;
		}

		syscall::write(fd as i32, default_file.as_bytes());
		syscall::close(fd as i32);
	}

	syscall::print("java project has been created\n");
}

pub fn run(_content: &str, parts: &[&str]) {
	let input = if parts.len() >= 2 {
		parts[1]
	} else {
		"Main.java"
	};

	let class_name = input.strip_suffix(".java").unwrap_or(input);

	let status = common::run_command("javac", &[input]);

	if status != 0 {
		syscall::print("compile failed\n");
		return;
	}

	common::run_command("java", &[class_name]);
}
