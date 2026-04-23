#![allow(dead_code)]

use crate::syscall;
use crate::syscall::fs::*;
use crate::syscall::nums::*;
use crate::syscall::proc::*;

use super::c;
use super::cpp;
use super::java;
use super::js;
use super::python;
use super::rust;

pub fn setup(parts: &[&str]) {
    let lang_str = parts[0]; // "cpp" or "python=3.10"

    let (lang, version) = if lang_str.contains('=') {
        let mut s = lang_str.splitn(2, '=');
        (s.next().unwrap_or(""), s.next().unwrap_or(""))
    } else {
        (lang_str, "")
    };

    match lang {
        "cpp" => cpp::setup(parts, version),
        "c" => c::setup(parts, version),
        "python" => python::setup(parts, version),
        "rust" => rust::setup(parts, version),
        "java" => java::setup(parts, version),
        "js" => js::setup(parts, version),
        _ => {
            syscall::print("unsupported language: ");
            syscall::print(lang);
            syscall::print("\n");
        }
    }
}

pub fn write_wshproject(lang: &str, version: &str) {
    // build content based on language and version
    let content = format!("type = {}\nversion = {}\n", lang, version);

    // null terminate the filename
    let filename = b".wshproject\0".to_vec();

    //open file
    let fd = syscall::open(filename.as_ptr(), O_WRONLY | O_CREAT | O_TRUNC, 0o644);

    //fail
    if fd < 0 {
        syscall::print("failed to create .wshproject\n");
        return;
    }

    syscall::write(fd as i32, content.as_bytes());
    syscall::close(fd as i32);
    syscall::print("created .wshproject\n");
}

pub fn run_project(parts: &[&str]) {
    match read_wshproject() {
        None => {
            syscall::print("no .wshproject found — run setproject first\n");
        }
        Some(content) => {
            // parse type from content
            // content looks like:
            // type = cpp
            // version = 17
            let project_type = content
                .lines()
                .find(|l| l.starts_with("type"))
                .and_then(|l| l.split('=').nth(1))
                .map(|s| s.trim())
                .unwrap_or("");

            match project_type {
                "cpp" => cpp::run(&content, &parts),
                "c" => c::run(&content, &parts),
                "python" => python::run(&content, &parts),
                "rust" => rust::run(&content, &parts),
                "java" => java::run(&content, &parts),
                "js" => js::run(&content, &parts),
                _ => {
                    syscall::print("unknown project type\n");
                }
            }
        }
    }
}

pub fn read_wshproject() -> Option<String> {
    let filename = b".wshproject\0".to_vec();
    let fd = syscall::open(filename.as_ptr(), O_RDONLY, 0);

    //failed to open file
    if fd < 0 {
        syscall::print("unable to locate .wshproject\n");
        return None;
    }

    let mut contents = [0u8; 256];
    let n = syscall::read(fd as i32, &mut contents);

    syscall::close(fd as i32);

    if n <= 0 {
        return None;
    }

    let content = core::str::from_utf8(&contents[..n as usize])
        .unwrap_or("")
        .to_string();

    Some(content)
}

pub fn run_command(program: &str, args: &[&str]) -> i32 {
    let path = match find_binary(program) {
        Some(p) => p,
        None => {
            syscall::print("command not found: ");
            syscall::print(program);
            syscall::print("\n");
            return -1;
        }
    };

    let mut path_bytes = path.into_bytes();
    path_bytes.push(b'\0');

    // argv[0] = program name (what user typed)
    let mut prog_cstring = program.as_bytes().to_vec();
    prog_cstring.push(b'\0');

    // argv[1..] = args
    let args_cstrings: Vec<Vec<u8>> = args
        .iter()
        .map(|a| {
            let mut v = a.as_bytes().to_vec();
            v.push(b'\0');
            v
        })
        .collect();

    let mut argv: Vec<*const u8> = Vec::new();
    argv.push(prog_cstring.as_ptr()); // <-- argv[0]
    argv.extend(args_cstrings.iter().map(|v| v.as_ptr()));
    argv.push(core::ptr::null());

    let (env_storage, envp) = get_envp();

    let pid = fork();
    if pid == 0 {
        execve(path_bytes.as_ptr(), argv.as_ptr(), envp.as_ptr());
        syscall::exit_group(1);
    }

    let mut wstatus: i32 = 0;
    if pid > 0 {
        wait4(pid, &mut wstatus as *mut i32, 0);
    }
    let _ = env_storage;
    let _ = prog_cstring; // keep alive until after wait
    wstatus
}

pub fn get_envp() -> (Vec<Vec<u8>>, Vec<*const u8>) {
    let fd = syscall::open(b"/proc/self/environ\0".as_ptr(), O_RDONLY, 0);
    let mut raw = [0u8; 8192];
    let n = syscall::read(fd as i32, &mut raw);
    syscall::close(fd as i32);

    if n <= 0 {
        return (Vec::new(), vec![core::ptr::null()]);
    }

    let mut env_storage: Vec<Vec<u8>> = Vec::new();
    let mut start = 0usize;
    for i in 0..n as usize {
        if raw[i] == 0 {
            if i > start {
                let mut entry = raw[start..i].to_vec();
                entry.push(0);
                env_storage.push(entry);
            }
            start = i + 1;
        }
    }

    let mut envp: Vec<*const u8> = Vec::new();
    for entry in &env_storage {
        envp.push(entry.as_ptr());
    }
    envp.push(core::ptr::null());

    (env_storage, envp)
}
