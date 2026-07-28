// no warnings even if the function is not used
#![allow(dead_code)]
// call consts
use crate::syscall::fd::{FdEntry, close_fd, track_fd};
use crate::syscall::nums::*;
use crate::syscall::structs::Stat;

// fs call defintion
pub fn print(s: &str) {
    syscall!(SYS_WRITE, 1u64, s.as_ptr() as u64, s.len() as u64);
}

pub fn close(fd: i32) {
    let _ = close_fd(fd);
}

pub(crate) fn close_raw(fd: i32) -> i64 {
    syscall!(SYS_CLOSE, fd as u64)
}

pub fn read(fd: i32, buf: &mut [u8]) -> i64 {
    syscall!(
        SYS_READ,
        fd as u64,
        buf.as_mut_ptr() as u64,
        buf.len() as u64
    )
}

pub fn write(fd: i32, buf: &[u8]) -> i64 {
    syscall!(SYS_WRITE, fd as u64, buf.as_ptr() as u64, buf.len() as u64)
}

pub fn chdir(path: *const u8) -> i64 {
    syscall!(SYS_CHDIR, path as u64)
}

pub fn fchdir(fd: i32) -> i64 {
    syscall!(SYS_FCHDIR, fd as u64)
}

pub fn getcwd(buf: &mut [u8]) -> *const u8 {
    let ret = syscall!(SYS_GETCWD, buf.as_mut_ptr() as u64, buf.len() as u64);
    ret as *const u8
}

pub fn stat(path: *const u8) -> i64 {
    let mut buf = Stat { _pad: [0u8; 144] };
    syscall!(SYS_STAT, path as u64, &mut buf as *mut Stat as u64)
}

fn read_env_var(name: &str) -> Option<String> {
    let fd = open(c"/proc/self/environ".as_ptr() as *const u8, O_RDONLY, 0);
    if fd < 0 {
        return None;
    } // failed to open /proc/self/environ fall back to default dirs

    let mut raw = [0u8; 8192];
    let n = read(fd as i32, &mut raw);
    close(fd as i32);

    if n <= 0 {
        return None;
    }

    let key = name.as_bytes();
    for entry in raw[..n as usize].split(|b| *b == 0) {
        if entry.starts_with(key) && entry.get(key.len()) == Some(&b'=') {
            let value = &entry[key.len() + 1..];
            if let Ok(s) = core::str::from_utf8(value) {
                return Some(s.to_string());
            }
        }
    }

    None
}

pub fn find_binary(name: &str) -> Option<String> {
    if name.contains('/') {
        let mut path_bytes = name.as_bytes().to_vec();
        path_bytes.push(0);

        if stat(path_bytes.as_ptr()) == 0 {
            return Some(name.to_string());
        }
        return None;
    }

    let path_env = read_env_var("PATH").unwrap_or_default();

    for dir in path_env.split(':') {
        let full_path = if dir.is_empty() {
            format!("./{}", name)
        } else {
            format!("{}/{}", dir, name)
        };

        let mut path_bytes = full_path.as_bytes().to_vec();
        path_bytes.push(0);

        if stat(path_bytes.as_ptr()) == 0 {
            return Some(full_path);
        }
    }

    let fallback_dirs = [
        ".",
        "/bin",
        "/usr/bin",
        "/usr/local/bin",
        "/sbin",
        "/usr/sbin",
    ];

    for dir in fallback_dirs.iter() {
        let full_path = format!("{}/{}", dir, name);

        let mut path_bytes = full_path.as_bytes().to_vec();
        path_bytes.push(0);

        if stat(path_bytes.as_ptr()) == 0 {
            return Some(full_path);
        }
    }
    None
}

pub fn pipe() -> Result<(i32, i32), i64> {
    let mut fds: [i32; 2] = [0, 0];
    let ret = syscall!(SYS_PIPE, fds.as_mut_ptr() as u64);
    if ret < 0 {
        return Err(ret);
    }

    let read_fd = track_fd(fds[0], FdEntry::Pipe);
    let write_fd = track_fd(fds[1], FdEntry::Pipe);
    Ok((read_fd, write_fd))
}

pub fn pipe2(flags: i32) -> Result<(i32, i32), i64> {
    let mut fds: [i32; 2] = [0, 0];
    let ret = syscall!(SYS_PIPE2, fds.as_mut_ptr() as u64, flags as u64);
    if ret < 0 {
        return Err(ret);
    }

    let read_fd = track_fd(fds[0], FdEntry::Pipe);
    let write_fd = track_fd(fds[1], FdEntry::Pipe);
    Ok((read_fd, write_fd))
}

pub fn dup2(oldfd: i32, newfd: i32) -> i64 {
    syscall!(SYS_DUP2, oldfd as u64, newfd as u64)
}

pub fn mkdir(path: *const u8, mode: u32) -> i64 {
    syscall!(SYS_MKDIR, path as u64, mode as u64)
}

pub fn open(path: *const u8, flags: i32, mode: u32) -> i64 {
    let fd = syscall!(SYS_OPEN, path as u64, flags as u64, mode as u64);
    if fd < 0 {
        fd
    } else {
        track_fd(fd as i32, FdEntry::File) as i64
    }
}
