#![allow(dead_code)]

use super::*;

fn print_socket_help() {
    syscall::print("socket commands:\n");
    syscall::print("  sockets --help            show socket command help\n");
    syscall::print("  opensocket <port>         open a TCP listening socket on a port\n");
    syscall::print("  accept <fd>               accept one client on a listening socket\n");
    syscall::print("  send <fd> [message]       send HTTP 200 (keeps client open by default)\n");
    syscall::print("  respond <fd> [message]    alias for send\n");
    syscall::print("  send -c|--close <fd> [msg] close client socket after response\n");
    syscall::print("  send -k|--keep-open <fd> [msg] keep client open after response\n");
    syscall::print("  sendraw <fd> <msg>        write raw bytes to a socket\n");
    syscall::print("  closesocket <fd>          close a tracked socket fd\n");
    syscall::print("  closesocket -c <fd>       close listener and accepted sockets from it\n");
}

fn write_socket_or_disconnect(fd: i32, buf: &[u8]) -> bool {
    let written = syscall::write(fd, buf);

    if written <= 0 {
        syscall::close(fd);
        syscall::print("socket disconnected, closed fd ");
        syscall::print(&fd.to_string());
        syscall::print("\n");
        return false;
    }

    if written < buf.len() as i64 {
        syscall::print("short write on fd ");
        syscall::print(&fd.to_string());
        syscall::print("\n");
    }

    true
}

pub fn run(parts: &[&str]) {
    match parts[0] {
        "sockets" => {
            if parts.len() >= 2 && parts[1] == "--help" {
                print_socket_help();
            } else {
                syscall::print("usage: sockets --help\n");
            }
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
        "sendraw" => {
            if parts.len() < 3 {
                syscall::print("usage: sendraw <client fd> <msg>\n");
            } else {
                let client_fd = parts[1].parse::<i32>().unwrap_or(0);
                let msg = parts[2..].join(" ");
                let _ = write_socket_or_disconnect(client_fd, msg.as_bytes());
            }
        }
        "send" | "respond" => {
            if parts.len() < 2 {
                syscall::print("usage: send [-c|--close|-k|--keep-open] <client fd> [message]\n");
            } else {
                let mut close_after_send = false;
                let mut client_fd: Option<i32> = None;
                let mut body_parts: Vec<&str> = Vec::new();

                for token in &parts[1..] {
                    if *token == "-c" || *token == "--close" {
                        close_after_send = true;
                        continue;
                    }
                    if *token == "-k" || *token == "--keep-open" {
                        close_after_send = false;
                        continue;
                    }

                    if client_fd.is_none() {
                        match token.parse::<i32>() {
                            Ok(fd) => client_fd = Some(fd),
                            Err(_) => {
                                syscall::print(
                                    "usage: send [-c|--close|-k|--keep-open] <client fd> [message]\n",
                                );
                                return;
                            }
                        }
                    } else {
                        body_parts.push(token);
                    }
                }

                let Some(client_fd) = client_fd else {
                    syscall::print(
                        "usage: send [-c|--close|-k|--keep-open] <client fd> [message]\n",
                    );
                    return;
                };

                let body = if !body_parts.is_empty() {
                    format!("{}\n", body_parts.join(" "))
                } else {
                    "Hello, wsh!\n".to_string()
                };
                let connection_header = if close_after_send {
                    "close"
                } else {
                    "keep-alive"
                };
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: {}\r\n\r\n{}",
                    body.len(),
                    connection_header,
                    body
                );
                if write_socket_or_disconnect(client_fd, response.as_bytes()) {
                    if close_after_send {
                        syscall::close(client_fd);
                        syscall::print("response sent, closed fd ");
                        syscall::print(&client_fd.to_string());
                        syscall::print("\n");
                    } else {
                        syscall::print("response sent, kept fd open ");
                        syscall::print(&client_fd.to_string());
                        syscall::print("\n");
                    }
                }
            }
        }
        "closesocket" => {
            if parts.len() < 2 {
                syscall::print("usage: closesocket [-c|--cascade] <fd>\n");
            } else {
                let mut idx = 1usize;
                let mut cascade = false;
                if parts[idx] == "-c" || parts[idx] == "--cascade" {
                    cascade = true;
                    idx += 1;
                }

                if parts.len() <= idx {
                    syscall::print("usage: closesocket [-c|--cascade] <fd>\n");
                    return;
                }

                let fd = parts[idx].parse::<i32>().unwrap_or(0);
                if cascade {
                    let mut closed = syscall::close_socket_cascade(fd);
                    if closed.is_empty() {
                        syscall::print("fd not tracked\n");
                    } else {
                        closed.sort();
                        syscall::print("closed fds:");
                        for entry_fd in closed {
                            syscall::print(" ");
                            syscall::print(&entry_fd.to_string());
                        }
                        syscall::print("\n");
                    }
                } else {
                    syscall::close(fd);
                    syscall::print("closed fd ");
                    syscall::print(&fd.to_string());
                    syscall::print("\n");
                }
            }
        }
        _ => {}
    }
}
