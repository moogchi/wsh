// call consts
use crate::syscall::nums::*;

// fs call defintion
pub fn print(s: &str) {
    syscall!(SYS_WRITE, 1u64, s.as_ptr() as u64, s.len() as u64);
}

pub fn close(fd: i32) {
    syscall!(SYS_CLOSE, fd as u64);
}
