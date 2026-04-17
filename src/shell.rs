// shell.rs
// main shell loop

use crate::commands;
use crate::syscall;

pub fn run() {
    loop {
        // print prompt
        syscall::print("wsh> ");

        //read input
        let mut buf = [0u8; 256];
        let n = syscall::read(0, &mut buf);

        // if ctrl d is press break
        if n <= 0 {
            break;
        }

        // trim the newline and convert to &str
        let input = &buf[..n as usize];
        let cmd = core::str::from_utf8(input).unwrap_or("").trim();

        commands::run(cmd);
    }
}
