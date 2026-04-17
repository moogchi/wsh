// commands.rs
#![allow(dead_code)]

use crate::syscall;
use crate::syscall::proc::*;

pub fn run(cmd: &str) {
    // match string to commands
    match cmd {
        "exit" => {
            syscall::exit_group(0);
        }
        "" => { // do nothing        
        }
        _ => {
            // change cmd into a mutable vector
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            let full_path = format!("/bin/{}", parts[0]);
            let mut path = full_path.as_bytes().to_vec();
            path.push(0);

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
