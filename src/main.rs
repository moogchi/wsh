mod commands;
mod shell;
mod syscall;

fn main() {
    let fd = syscall::socket(syscall::AF_INET, syscall::SOCK_STREAM, syscall::IPPROTO_TCP)
        .expect("socket failed");

    syscall::print("socket() -> fd ");
    syscall::print(&fd.to_string());
    syscall::print("\n");

    syscall::close(fd);
    syscall::print("close() → ok\n");
}
