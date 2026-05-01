// shell.rs
// main shell loop

use crate::commands;
use crate::syscall;

fn shutdown_shell() {
    let open_fds = syscall::cleanup_fds();
    if !open_fds.is_empty() {
        syscall::print("warning: closing open fds: ");
        syscall::print(&open_fds.len().to_string());
        syscall::print("\n");
    }
}

pub fn run() {
    loop {
        // print prompt

        // get the cwd onto the buffer
        let mut cwd_buf = [0u8; 256];
        syscall::getcwd(&mut cwd_buf);

        // translate cwd buffer to string.
        let cwd = unsafe {
            core::ffi::CStr::from_ptr(cwd_buf.as_ptr() as *const i8)
                .to_str()
                .unwrap_or("?")
        };

        //covert so home is ~
        let display = if cwd.starts_with("/home/sihoon") {
            cwd.replacen("/home/sihoon", "~", 1)
        } else {
            cwd.to_string()
        };

        syscall::print("[wsh] ");
        syscall::print(&display);
        syscall::print(" » ");

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

    shutdown_shell();
}
