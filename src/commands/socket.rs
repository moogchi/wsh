#![allow(dead_code)]

use super::*;

pub fn run(parts: &[&str]) {
    match parts[0] {
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
        "sendraw" => {
            if parts.len() < 3 {
                syscall::print("usage: sendraw <client fd> <msg>\n");
            } else {
                let client_fd = parts[1].parse::<i32>().unwrap_or(0);
                let msg = parts[2..].join(" ");
                syscall::write(client_fd, msg.as_bytes());
            }
        }
        "send" => {
            if parts.len() < 2 {
                syscall::print("usage: send <client fd> [message]\n");
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
        _ => {}
    }
}
