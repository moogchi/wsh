#![allow(dead_code)]

use super::*;
use crate::commands::projects::common;

pub fn run(parts: &[&str]) {
    match parts[0] {
        "exit" => {
            syscall::exit_group(0);
        }
        "cd" => {
            let target = if parts.len() < 2 {
                "/home/sihoon\0".as_bytes().to_vec()
            } else {
                let mut p = parts[1].as_bytes().to_vec();
                p.push(0);
                p
            };
            chdir(target.as_ptr());
        }
        "pwd" => {
            let mut pwd_buf = [0u8; 256];
            syscall::getcwd(&mut pwd_buf);
            let pwd = unsafe {
                core::ffi::CStr::from_ptr(pwd_buf.as_ptr() as *const i8)
                    .to_str()
                    .unwrap_or("?")
            };
            syscall::print(pwd);
            syscall::print("\n");
        }
        "echo" => {
            let echoed = parts[1..].join(" ");
            syscall::print(&echoed);
            syscall::print("\n");
        }
        _ => {
            common::run_command(parts[0], &parts[1..]);
        }
    }
}
