// no warnings even if the function is not used
#![allow(dead_code)]
// call consts
use crate::syscall::nums::*;

//proc call defintion
pub fn exit_group(status: i64) -> ! {
    syscall!(SYS_EXIT_GROUP, status);
    unreachable!("exit_group syscall does not return")
}

pub fn fork() -> i32 {
    let pid = syscall!(SYS_FORK, 0u64);
    pid as i32
}

pub fn execve(path: *const u8, argv: *const *const u8, envp: *const *const u8) -> i64 {
    syscall!(SYS_EXECVE, path as u64, argv as u64, envp as u64)
}

pub fn wait4(pid: i32, wstatus: *mut i32, options: u64) -> i32 {
    syscall!(SYS_WAIT4, pid as u64, wstatus as u64, options, 0u64) as i32
}
