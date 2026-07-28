//const
pub const SYS_READ: u64 = 0;
pub const SYS_WRITE: u64 = 1;
pub const SYS_OPEN: u64 = 2;
pub const SYS_CLOSE: u64 = 3;
pub const SYS_STAT: u64 = 4;
pub const SYS_DUP2: u64 = 33;
pub const SYS_SOCKET: u64 = 41;
pub const SYS_PIPE: u64 = 22;
pub const SYS_ACCEPT: u64 = 43;
pub const SYS_BIND: u64 = 49;
pub const SYS_LISTEN: u64 = 50;
pub const SYS_SETSOCKOPT: u64 = 54;
pub const SYS_FORK: u64 = 57;
pub const SYS_EXECVE: u64 = 59;
pub const SYS_WAIT4: u64 = 61;
pub const SYS_GETCWD: u64 = 79;
pub const SYS_CHDIR: u64 = 80;
pub const SYS_FCHDIR: u64 = 80;
pub const SYS_MKDIR: u64 = 83;
pub const SYS_EXIT_GROUP: u64 = 231;
pub const SYS_PIPE2: u64 = 293;

pub const AF_INET: u64 = 2;
pub const SOCK_STREAM: u64 = 1;
pub const IPPROTO_TCP: u64 = 6;
pub const SOL_SOCKET: i32 = 1;
pub const SO_REUSEADDR: i32 = 2;

//open const
pub const O_RDONLY: i32 = 0;
pub const O_WRONLY: i32 = 1; // write only
pub const O_CREAT: i32 = 64; // create if doesn't exist
pub const O_TRUNC: i32 = 512; // truncate to 0 if exists
