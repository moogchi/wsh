# Wustite / wsh Roadmap

## wsh — Wustite Shell

---

## Done

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
- [x] setproject (python/cpp/rust/c/java/js)
- [x] project-aware touch (starter code per language)
- [x] project-aware run/compile
- [x] python venv auto-setup
- [x] quoted string parsing ("hello world" as one arg)

---

## Core Architecture (NEW)

- [ ] FD table (`HashMap<i32, FdHandle>`)
- [ ] `FdHandle` struct with `Drop` (auto-close on removal)
- [ ] `FDEntry` enum:
  - [ ] File
  - [ ] Socket
  - [ ] Pipe
- [ ] centralized FD lifecycle management
- [ ] prevent double-close (only close via FD table removal)
- [ ] shell shutdown cleanup (close all remaining FDs)
- [ ] exit warning for open FDs

---

## FD / Dev Observability

- [ ] `fds` command (list all open FDs)
- [ ] `fds <fd>` (detailed info)
- [ ] FD table pretty print (type, state, metadata)
- [ ] socket state tracking (LISTENING / CONNECTED)
- [ ] peer/local address display for sockets
- [ ] optional: `watch fds` (print updates on change)

---

## I/O Model Cleanup

- [ ] unify I/O around `read` / `write`
- [ ] rename:
  - [ ] `send` → `write`
  - [ ] `respond` → HTTP helper (not core I/O)
- [ ] `writehex` or `writeraw` for binary data
- [ ] consistent interface across:
  - [ ] files
  - [ ] sockets
  - [ ] pipes

---

## Socket System Improvements

- [ ] track socket metadata in FD table
- [ ] `bind <fd> <ip:port>`
- [ ] `listen <fd> <backlog>`
- [ ] `accept` returns tracked FD with metadata
- [ ] unify `closesocket` → `close`
- [ ] HTTP helper:
  - [ ] `respond <fd> "body"` sends valid HTTP/1.1 response
- [ ] improved error handling (port in use, invalid FD, etc.)

---

## Developer Mode (REWORK)

- [ ] `enable --dev-mode --confirm`
- [ ] dev prompt: `wsh(dev) »`
- [ ] dev command namespace:
  - [ ] `dev syscall <name> ...`
  - [ ] `dev trace <command>`
- [ ] syscall result formatting:
  - [ ] return value
  - [ ] errno decoding
- [ ] restrict syscall whitelist:
  - [ ] open, read, write, close
  - [ ] socket-related
  - [ ] stat/access
- [ ] FD table auto-update on syscalls

---

## Safe Mode

- [ ] path-based write restrictions (not just command-based)
- [ ] block writes outside allowed directory (~/ or project)
- [ ] confirmation prompts for:
  - [ ] recursive deletes
  - [ ] overwrites
- [ ] restrict low ports (<1024)
- [ ] disable dangerous syscalls when safe mode is enabled

---

## Must Have

- [ ] pipe support (ls | grep foo)
- [ ] redirect (> and >>)
- [ ] up arrow command history
- [ ] multiline commands (backslash at end of line)
- [ ] command timing (show execution time after each command)

---

## Process & Execution

- [ ] background jobs (`&`)
- [ ] job tracking
- [ ] `jobs` command
- [ ] `fg` / `bg`
- [ ] signal handling (SIGINT, SIGCHLD)

---

## Project System

- [ ] `.wshproject` config file (python env, cpp split into src header)
- [ ] custom run/build commands per project
- [ ] project env variables
- [ ] auto-detect project type
- [ ] project-specific dependencies

---

## Debug / Introspection Tools

- [ ] `errno` command (print last error)
- [ ] `time <command>` (detailed timing)
- [ ] `trace` (strace-lite for shell commands)
- [ ] verbose mode (`--verbose`)

---

## UX / Polish

- [ ] better error messages (human-readable errno)
- [ ] command suggestions (did you mean…)
- [ ] consistent usage messages
- [ ] colored output (errors, warnings, success)
- [ ] command grouping in help menu

---

## Nice To Have

- [ ] setproject python: support specific versions via pyenv
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
- [ ] termios raw mode (ctrl+a, ctrl+e, ctrl+l, ctrl+r)
- [ ] prlimit64 (raise fd limit for many connections)
- [ ] respondall (send to all connected clients)

---

## Networking Tools

- [ ] simple HTTP server mode
- [ ] `serve <port>` shortcut
- [ ] `connect <ip:port>` (client mode)
- [ ] broadcast improvements (`respondall`)

---

## Low Priority

- [ ] tab completion (raw terminos)
- [ ] IPv6 support (SockAddrIn6, AF_INET6)
- [ ] full Stat struct (for ls -l)
- [ ] environment variable support
- [ ] socket fd → port mapping
- [ ] ping built-in (raw ICMP)
- [ ] dns lookup built-in
- [ ] http GET built-in

---

## Stretch Goals

- [ ] pipe syscall integration into shell pipes
- [ ] `select` / `poll` for multiple sockets
- [ ] interactive syscall REPL
- [ ] record/replay syscall sessions

---
