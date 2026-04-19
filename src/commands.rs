// commands.rs
#![allow(dead_code)]

use crate::syscall;
use crate::syscall::fs::*;
use crate::syscall::proc::*;

pub fn run(cmd: &str) {
    // change cmd into a mutable vector
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

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
            let full_path = format!("/bin/{}", parts[0]);
            let mut path = full_path.as_bytes().to_vec();
            path.push(0);

            // match string to commands

            // null-terminate each argument and store them so they stay alive in memory
            // we can't just take pointers in a loop because the vecs would be dropped
            // at the end of each iteration, leaving dangling pointers (use-after-free)
            let args_cstrings: Vec<Vec<u8>> = parts
                .iter()
                .map(|arg| {
                    let mut v = arg.as_bytes().to_vec();
                    v.push(b'\0');
                    v
                })
                .collect();

            // build argv as an array of pointers into args_cstrings, null terminated
            // argv[0] = program name, argv[1..] = arguments, argv[last] = NULL
            let mut argv: Vec<*const u8> = args_cstrings.iter().map(|v| v.as_ptr()).collect();
            argv.push(core::ptr::null());

            let pid = fork();

            //child process
            if pid == 0 {
                execve(path.as_ptr(), argv.as_ptr(), core::ptr::null());
                // if we get here the execve has failed
                syscall::exit_group(1);
            } else if pid > 0 {
                // parent process
                wait4(pid, core::ptr::null_mut(), 0);
            } else if pid < 0 {
                syscall::print("Fork Failed \n");
            }
        }
    }
}
