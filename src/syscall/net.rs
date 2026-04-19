// no warnings even if the function is not used
#[allow(dead_code)]
// call consts
use crate::syscall::nums::*;
use crate::syscall::structs::SockAddrIn;
use core::mem;

//net call defintion
pub fn socket(domain: u64, typ: u64, protocol: u64) -> Result<i32, i64> {
    let fd = syscall!(SYS_SOCKET, domain, typ, protocol);
    if fd < 0 { Err(-fd) } else { Ok(fd as i32) }
}

fn htons(x: u16) -> u16 {
    x.to_be()
}

// TODO: add IPv6 support via SockAddrIn6 and AF_INET6
pub fn bind(sockfd: i32, port: u16) -> Result<(), i64> {
    let sockaddr = SockAddrIn {
        sin_family: AF_INET as u16,
        sin_port: htons(port),
        sin_addr: 0, // INADDR_ANY
        sin_zero: [0; 8],
    };
    let ret = syscall!(
        SYS_BIND,
        sockfd as u64,
        &sockaddr as *const SockAddrIn as u64,
        mem::size_of::<SockAddrIn>() as u64
    );
    if ret < 0 { Err(-ret) } else { Ok(()) }
}

pub fn listen(sockfd: i32, backlog: i32) -> Result<(), i64> {
    let ret = syscall!(SYS_LISTEN, sockfd as u64, backlog as u64);
    if ret < 0 { Err(-ret) } else { Ok(()) }
}

//TODO: client connection
pub fn accept(sockfd: i32) -> Result<i32, i64> {
    let fd = syscall!(SYS_ACCEPT, sockfd as u64, 0u64, 0u64);
    if fd < 0 { Err(-fd) } else { Ok(fd as i32) }
}

pub fn setsockopt(sockfd: i32, level: i32, optname: i32, optval: *const i32) -> Result<(), i64> {
    let ret = syscall!(
        SYS_SETSOCKOPT,
        sockfd as u64,
        level as u64,
        optname as u64,
        optval as u64,
        mem::size_of::<i32>() as u64
    );
    if ret < 0 { Err(-ret) } else { Ok(()) }
}
