#![allow(dead_code)]

use super::*;
use crate::commands::projects::common;

fn list_fds(parts: &[&str]) {
    let mut open_fds = syscall::tracked_fds();
    open_fds.sort_by_key(|(fd, _)| *fd);

    if parts.len() == 1 {
        if open_fds.is_empty() {
            syscall::print("no tracked fds\n");
            return;
        }

        for (fd, entry) in open_fds {
            syscall::print("fd ");
            syscall::print(&fd.to_string());
            syscall::print(": ");
            syscall::print(entry.label());
            syscall::print("\n");
        }
        return;
    }

    let target_fd = parts[1].parse::<i32>().unwrap_or(-1);
    if let Some((fd, entry)) = open_fds.into_iter().find(|(fd, _)| *fd == target_fd) {
        syscall::print("fd ");
        syscall::print(&fd.to_string());
        syscall::print(": ");
        syscall::print(entry.label());
        syscall::print("\n");
    } else {
        syscall::print("fd not tracked\n");
    }
}

fn shutdown_with_fd_cleanup(status: i64) -> ! {
    let open_fds = syscall::cleanup_fds();
    if !open_fds.is_empty() {
        syscall::print("warning: closing open fds: ");
        syscall::print(&open_fds.len().to_string());
        syscall::print("\n");
    }
    syscall::exit_group(status);
}

pub fn run(parts: &[&str]) {
    match parts[0] {
        "exit" => {
            shutdown_with_fd_cleanup(0);
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
        "fds" => {
            list_fds(parts);
        }
        _ => {
            common::run_command(parts[0], &parts[1..]);
        }
    }
}
