// commands.rs
#![allow(dead_code)]

use crate::syscall;
use crate::syscall::fs::*;
use crate::syscall::nums::*;
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
        "opensocket" => {
            if parts.len() < 2 {
                syscall::print("usage: opensocket <port> \n");
            } else {
                let port = parts[1].parse::<u16>().unwrap_or(0);
                let fd = syscall::socket(AF_INET, SOCK_STREAM, IPPROTO_TCP).unwrap_or(-1);
                let one: i32 = 1;
                if let Err(e) =
                    syscall::setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &one as *const i32)
                {
                    syscall::print("Set Sock Opt failed: ");
                    syscall::print(&e.to_string());
                    syscall::print("\n");
                    return;
                }
                if let Err(e) = syscall::bind(fd, port) {
                    syscall::print("bind failed: ");
                    syscall::print(&e.to_string());
                    syscall::print("\n");
                    return;
                }
                if let Err(e) = syscall::listen(fd, 128) {
                    syscall::print("bind failed: ");
                    syscall::print(&e.to_string());
                    syscall::print("\n");
                    return;
                }
                syscall::print("listening on port ");
                syscall::print(parts[1]);
                syscall::print(" fd: ");
                syscall::print(&fd.to_string());
                syscall::print("\n");
            }
        }
        "accept" => {
            if parts.len() < 2 {
                syscall::print("usage: accept <fd> \n");
            } else {
                let fd = parts[1].parse::<i32>().unwrap_or(0);
                match syscall::accept(fd) {
                    Ok(client_fd) => {
                        syscall::print("client connected, fd: ");
                        syscall::print(&client_fd.to_string());
                        syscall::print("\n");
                    }
                    Err(e) => {
                        syscall::print("accept failed: ");
                        syscall::print(&e.to_string());
                        syscall::print("\n");
                    }
                }
            }
        }
        "send" => {
            if parts.len() < 3 {
                syscall::print("usage: send <client fd> <msg>\n");
            } else {
                let client_fd = parts[1].parse::<i32>().unwrap_or(0);
                let msg = parts[2..].join(" ");
                syscall::write(client_fd, msg.as_bytes());
            }
        }
        "respond" => {
            if parts.len() < 2 {
                syscall::print("usage: respond <client fd> [message]\n");
            } else {
                let client_fd = parts[1].parse::<i32>().unwrap_or(0);
                let body = if parts.len() > 2 {
                    format!("{}\n", parts[2..].join(" "))
                } else {
                    "Hello, wsh!\n".to_string()
                };
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                syscall::write(client_fd, response.as_bytes());
            }
        }
        "closesocket" => {
            if parts.len() < 2 {
                syscall::print("usage: closesocket <fd>\n");
            } else {
                let fd = parts[1].parse::<i32>().unwrap_or(0);
                syscall::close(fd);
                syscall::print("closed fd ");
                syscall::print(parts[1]);
                syscall::print("\n");
            }
        }
        _ => {
            // path finding we need to loop through and find the bin files

            match find_binary(parts[0]) {
                Some(path) => {
                    // match string to commands

                    // use path_bytes.as_ptr()
                    let mut path_bytes = path.into_bytes();
                    path_bytes.push(b'\0');

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
                    let mut argv: Vec<*const u8> =
                        args_cstrings.iter().map(|v| v.as_ptr()).collect();
                    argv.push(core::ptr::null());

                    let pid = fork();

                    //child process
                    if pid == 0 {
                        let env_term = b"TERM=xterm\0";
                        let envp = [env_term.as_ptr(), core::ptr::null()];
                        execve(path_bytes.as_ptr(), argv.as_ptr(), envp.as_ptr());
                        // if we get here the execve has failed
                        syscall::exit_group(1);
                    } else if pid > 0 {
                        // parent process
                        wait4(pid, core::ptr::null_mut(), 0);
                    } else if pid < 0 {
                        syscall::print("Fork Failed \n");
                    }
                }
                None => {
                    syscall::print("command not found\n");
                }
            }
        }
    }
}
