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
- [x] bind, listen, accept, setsockopt
- [x] opensocket, accept, send, respond, closesocket commands
- [x] commands split into builtins.rs and socket.rs

### Must Have

- [ ] quoted string parsing ("hello world" as one arg)
- [ ] pipe support (ls | grep foo)
- [ ] redirect (> and >>)
- [ ] up arrow command history
- [ ] multiline commands (backslash at end of line)
- [ ] command timing (show execution time after each command)
- [ ] setproject (python/cpp/rust)
- [ ] project-aware touch (starter code per language)
- [ ] project-aware run/compile
- [ ] python venv auto-setup

### Nice To Have

- [ ] .wshrc config file
- [ ] prompt.path = full | short | minimal
- [ ] prompt.show_host
- [ ] prompt.show_git (reads .git/HEAD directly)
- [ ] prompt.color (green/red based on exit code)
- [ ] cd - (return to previous directory)
- [ ] clear built-in
- [ ] which built-in
- [ ] history built-in
- [ ] aliases
- [ ] env built-in
- [ ] hexdump built-in
- [ ] wsh scripting (.wsh files)
- [ ] termios raw mode (unlocks ctrl+a, ctrl+e, ctrl+l, ctrl+r)
- [ ] prlimit64 (raise fd limit for many connections)
- [ ] respondall (send to all connected clients)
- [ ] safe mode (--safe flag or .wshrc)
- [ ] safe_mode.write_boundary = ~/
- [ ] safe_mode.min_port = 1024
- [ ] safe_mode.block_rm
- [ ] developer mode (type "develop" 5x to unlock)
- [ ] dev: raw syscall command (syscall <args...>)
- [ ] dev: prompt changes to wsh(dev) »

### Low Priority

- [ ] tab completion
- [ ] IPv6 support (SockAddrIn6, AF_INET6)
- [ ] full Stat struct (for ls -l)
- [ ] environment variable support
- [ ] socket fd→port mapping for closesocket
- [ ] ping built-in (raw ICMP)
- [ ] dns lookup built-in
- [ ] http GET built-in
