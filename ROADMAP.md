# Wustite / wsh Roadmap

## wsh — Wustite Shell

### Done

- [x] raw syscall macro (all 6 arities, inline asm)
- [x] socket, read, write, close, print
- [x] fork, execve, wait4, exit_group
- [x] shell loop with custom prompt
- [x] ~ substitution in prompt
- [x] cd, pwd, echo built-ins
- [x] PATH resolution (/bin, /usr/bin, etc)
- [x] TERM=xterm env fix for programs like git

### In Progress

- [ ] bind, listen, accept (socket commands)
- [ ] opensocket built-in

### Todo

- [ ] quoted string parsing ("hello world" as one arg)
- [ ] pipe support (ls | grep foo)
- [ ] tab completion
- [ ] run built-in (auto detect project type)
- [ ] compile built-in
- [ ] redirect (> and >>)
- [ ] environment variable support
- [ ] full Stat struct (for ls -l)
- [ ] prlimit64 (raise fd limit for many connections)

## kernel — Wustite Kernel

- [ ] TBD — RISC-V or x86-64
- [ ] boot sequence
- [ ] memory management
- [ ] scheduler
- [ ] POSIX syscall interface
- [ ] network stack
