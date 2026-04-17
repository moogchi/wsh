// call consts
use crate::syscall::nums::*;

//net call defintion
pub fn socket(domain: u64, typ: u64, protocol: u64) -> Result<i32, i64> {
    let fd = syscall!(SYS_SOCKET, domain, typ, protocol);
    if fd < 0 { Err(-fd) } else { Ok(fd as i32) }
}
