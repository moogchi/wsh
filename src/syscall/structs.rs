#![allow(dead_code)]

#[repr(C)]
pub struct Stat {
    pub _pad: [u8; 144],
}
// TODO: define full Stat struct fields when implementing ls -l

#[repr(C)]
pub struct SockAddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: u32,
    pub sin_zero: [u8; 8],
}
