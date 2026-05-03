# Examples

This page contains examples of how to use these commands

## File Descriptor

**fds**

```wsh
[wsh] ~/Coding/wustite/wsh » fds
no tracked fds
```

## Socket commands

### opensocket

```wsh
[wsh] ~/Coding/wustite/wsh » opensocket 8000
listening on port 8000 fd: 3
```

### accept

Client Terminal:

```bash
nc 127.0.0.1 8000
```

Server terminal:

```wsh
[wsh] ~/Coding/wustite/wsh » fds
fd 3: socket(listening)
[wsh] ~/Coding/wustite/wsh » accept 3
client connected, fd: 4
[wsh] ~/Coding/wustite/wsh » fds
fd 3: socket(listening)
fd 4: socket(connected)
```

### send/respond

Client terminal:

```bash
nc 127.0.0.1 8000

HTTP/1.1 200 OK
Content-Length: 5
Connection: keep-alive

what
HTTP/1.1 200 OK
Content-Length: 4
Connection: close

bye
```

Server terminal:

```wsh
[wsh] ~/Coding/wustite/wsh » send 4 "what"
response sent, kept fd open 4
[wsh] ~/Coding/wustite/wsh » send -c 4 "bye"
response sent, closed fd 4
```

You can also put the flag at the end:

```wsh
[wsh] ~/Coding/wustite/wsh » send 4 "bye" --close
response sent, closed fd 4
```

### closesocket

Close one fd:

```wsh
[wsh] ~/Coding/wustite/wsh » closesocket 3
closed fd 3
```

Cascade close from listener fd:

```wsh
[wsh] ~/Coding/wustite/wsh » fds
fd 3: socket(listening)
fd 4: socket(connected)
fd 5: socket(connected)
[wsh] ~/Coding/wustite/wsh » closesocket -c 3
closed fds: 3 4 5
[wsh] ~/Coding/wustite/wsh » fds
no tracked fds
```

## Core shell commands

### pwd and cd

```wsh
[wsh] ~/Coding/wustite/wsh » pwd
/home/sihoon/Coding/wustite/wsh
[wsh] ~/Coding/wustite/wsh » cd test
[wsh] ~/Coding/wustite/wsh/test » pwd
/home/sihoon/Coding/wustite/wsh/test
```

### echo

```wsh
[wsh] ~/Coding/wustite/wsh » echo hello world
hello world
[wsh] ~/Coding/wustite/wsh » echo "hello from wsh"
hello from wsh
```

### fds single lookup

```wsh
[wsh] ~/Coding/wustite/wsh » fds 3
fd 3: socket(listening)
```

## Project commands

### setproject in current directory

```wsh
[wsh] ~/Coding/wustite/wsh » setproject rust
created .wshproject
```

### openproject creates folder + setup

```wsh
[wsh] ~/Coding/wustite » openproject demo cpp=17
created .wshproject
```

### run uses .wshproject type

```wsh
[wsh] ~/Coding/wustite/demo » run
... project-specific build/run output ...
```
