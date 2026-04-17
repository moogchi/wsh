// no warnings even if the function is not used
#![allow(dead_code)]
// call consts
use crate::syscall::nums::*;

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
