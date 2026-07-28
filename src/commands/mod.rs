// commands/mod.rs
pub mod builtins;
pub mod projects;
pub mod socket;

use crate::syscall;
use crate::syscall::fs::*;
use crate::syscall::nums::*;
use crate::syscall::proc::*;

fn tokenize(cmd: &str) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;
    let mut escape_next = false;
    let mut token_started = false;

    for ch in cmd.chars() {
        if escape_next {
            current.push(ch);
            token_started = true;
            escape_next = false;
            continue;
        }

        match ch {
            '\\' => {
                if in_single {
                    current.push(ch);
                    token_started = true;
                } else {
                    escape_next = true;
                    token_started = true;
                }
            }
            '\'' if !in_double => {
                in_single = !in_single;
                token_started = true;
            }
            '"' if !in_single => {
                in_double = !in_double;
                token_started = true;
            }
            c if c.is_whitespace() && !in_single && !in_double => {
                if token_started {
                    parts.push(core::mem::take(&mut current));
                    token_started = false;
                }
            }
            '|' if !in_single && !in_double => {
                if token_started {
                    parts.push(core::mem::take(&mut current));
                    token_started = false;
                }
                parts.push("|".to_string());
            }
            _ => {
                current.push(ch);
                token_started = true;
            }
        }
    }

    // Keep trailing backslash as a literal if it had nothing to escape.
    if escape_next {
        current.push('\\');
    }

    if token_started {
        parts.push(current);
    }

    parts
}

fn dispatch(parts: &[&str]) {
    match parts[0] {
        "cd" | "pwd" | "echo" | "exit" => builtins::run(parts),
        "opensocket" | "accept" | "send" | "respond" | "closesocket" => socket::run(parts),
        "setproject" | "openproject" | "run" => projects::run(parts),
        _ => builtins::run(parts),
    }
}

fn split_pipeline<'a>(parts: &[&'a str]) -> Vec<Vec<&'a str>> {
    let mut stages: Vec<Vec<&str>> = Vec::new();
    let mut current: Vec<&str> = Vec::new();

    for &part in parts {
        if part == "|" {
            stages.push(core::mem::take(&mut current));
        } else {
            current.push(part);
        }
    }
    stages.push(current);

    stages
}

fn run_pipeline(stages: &[Vec<&str>]) {
    if stages.iter().any(|stage| stage.is_empty()) {
        syscall::print("wsh: syntax error near '|'\n");
        return;
    }

    let mut prev_read: Option<i32> = None;
    let mut pids: Vec<i32> = Vec::with_capacity(stages.len());
    let last = stages.len() - 1;

    for (i, stage) in stages.iter().enumerate() {
        let pipe_fds = if i != last {
            match syscall::pipe() {
                Ok(fds) => Some(fds),
                Err(_) => {
                    syscall::print("wsh: pipe failed\n");
                    return;
                }
            }
        } else {
            None
        };

        let pid = fork();
        if pid == 0 {
            if let Some(read_fd) = prev_read {
                syscall::dup2(read_fd, 0);
                syscall::close(read_fd);
            }
            if let Some((read_fd, write_fd)) = pipe_fds {
                syscall::close(read_fd);
                syscall::dup2(write_fd, 1);
                syscall::close(write_fd);
            }
            dispatch(stage);
            syscall::exit_group(0);
        }

        if let Some(read_fd) = prev_read {
            syscall::close(read_fd);
        }

        prev_read = if let Some((read_fd, write_fd)) = pipe_fds {
            syscall::close(write_fd);
            Some(read_fd)
        } else {
            None
        };

        pids.push(pid);
    }

    for pid in pids {
        let mut status: i32 = 0;
        wait4(pid, &mut status as *mut i32, 0);
    }
}

pub fn run(cmd: &str) {
    let owned_parts = tokenize(cmd);
    let parts: Vec<&str> = owned_parts.iter().map(String::as_str).collect();

    if parts.is_empty() {
        return;
    }

    let stages = split_pipeline(&parts);
    if stages.len() <= 1 {
        dispatch(&parts);
        return;
    }

    run_pipeline(&stages);
}

#[cfg(test)]
mod tests {
    use super::tokenize;

    #[test]
    fn tokenize_simple_whitespace() {
        assert_eq!(tokenize("echo hello world"), vec!["echo", "hello", "world"]);
    }

    #[test]
    fn tokenize_double_quoted_arg() {
        assert_eq!(
            tokenize("echo \"hello world\""),
            vec!["echo", "hello world"]
        );
    }

    #[test]
    fn tokenize_single_quoted_arg() {
        assert_eq!(tokenize("echo 'hello world'"), vec!["echo", "hello world"]);
    }

    #[test]
    fn tokenize_keeps_empty_quoted_arg() {
        assert_eq!(tokenize("echo \"\" after"), vec!["echo", "", "after"]);
    }

    #[test]
    fn tokenize_escaped_space() {
        assert_eq!(tokenize("echo hello\\ world"), vec!["echo", "hello world"]);
    }

    #[test]
    fn tokenize_escaped_quote_in_double_quotes() {
        assert_eq!(
            tokenize("echo \"say \\\"hi\\\"\""),
            vec!["echo", "say \"hi\""]
        );
    }
}
