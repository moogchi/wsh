# WSH - Wustite Shell

Small Rust shell project focused on Unix-y fd workflows.

If you want the bigger plan (and the half-outdated plan), roadmap is here: [ROADMAP.md](ROADMAP.md)

## What this shell is for

The main point of wsh is poking around file descriptors without a lot of ceremony.
Open sockets, accept clients, inspect tracked fds, close things cleanly, and run project helpers.

## Setup

Only thing you really need is Rust.
For socket testing, netcat is super useful: [netcat guide](https://www.baeldung.com/linux/netcat-command)

```bash
git clone https://github.com/moogchi/wsh.git
cd wsh
cargo run
```

Or with make:

```bash
make run
```

## Command List

Full examples are in [EXAMPLE.md](EXAMPLE.md).

### Core shell

```wsh
cd [path]
pwd
echo <text>
exit
fds
fds <fd>
```

Notes:

- fds prints only tracked descriptors managed by wsh.
- exit auto-cleans tracked fds and warns if anything was left open.

### Socket commands

```wsh
sockets --help
opensocket <port>
accept <listener_fd>
send [-c|--close|-k|--keep-open] <client_fd> [message]
respond [-c|--close|-k|--keep-open] <client_fd> [message]
sendraw <client_fd> <message>
closesocket <fd>
closesocket -c <listener_fd>
```

Socket behavior details:

- closesocket <listener_fd> closes only that listener.
- accepted clients stay open unless explicitly closed.
- closesocket -c <listener_fd> cascades and closes listener + connected sockets that came from it.
- send and respond are aliases.

### Project helpers

```wsh
setproject <lang>[=version]
openproject <name> <lang>[=version]
run
```

Supported langs right now: c, cpp, python, rust, java, js.

## Quick socket flow

```wsh
opensocket 8000
accept 3
send 4 "hello"
send -c 4 "bye"
closesocket -c 3
```

## Current state

This is still actively hacked on.
Things are functional, but a few rough edges are expected while commands are evolving.
