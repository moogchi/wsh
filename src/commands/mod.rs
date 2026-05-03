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

pub fn run(cmd: &str) {
    let owned_parts = tokenize(cmd);
    let parts: Vec<&str> = owned_parts.iter().map(String::as_str).collect();

    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "cd" | "pwd" | "echo" | "exit" => builtins::run(&parts),
        "opensocket" | "accept" | "send" | "respond" | "closesocket" => socket::run(&parts),
        "setproject" | "openproject" | "run" => projects::run(&parts),
        _ => builtins::run(&parts),
    }
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
        assert_eq!(tokenize("echo \"hello world\""), vec!["echo", "hello world"]);
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
