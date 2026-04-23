// no warnings even if the function is not used
#![allow(dead_code)]
// call consts
use crate::syscall::nums::*;
use crate::syscall::structs::Stat;

// fs call defintion
pub fn print(s: &str) {
    syscall!(SYS_WRITE, 1u64, s.as_ptr() as u64, s.len() as u64);
}

pub fn close(fd: i32) {
    syscall!(SYS_CLOSE, fd as u64);
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

pub fn find_binary(name: &str) -> Option<String> {
    let dirs = [
        ".",
        "/bin",
        "/usr/bin",
        "/usr/local/bin",
        "/sbin",
        "/usr/sbin",
    ];

    for dir in dirs.iter() {
        let full_path = format!("{}/{}", dir, name);

        let mut path_bytes = full_path.as_bytes().to_vec();
        path_bytes.push(0);

        let ret = stat(path_bytes.as_ptr());

        if ret == 0 {
            return Some(full_path);
        }
    }
    None
}

pub fn mkdir(path: *const u8, mode: u32) -> i64 {
    syscall!(SYS_MKDIR, path as u64, mode as u64)
}

pub fn open(path: *const u8, flags: i32, mode: u32) -> i64 {
    syscall!(SYS_OPEN, path as u64, flags as u64, mode as u64)
}
