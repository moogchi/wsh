#define _POSIX_C_SOURCE 200809L

#include "common.h"

#include <errno.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/select.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h> /* fork, execl, pipe, close, read, write, usleep */

static void die(const char *msg) {
    perror(msg);
    exit(1);
}

static long long now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (long long)ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

wsh_proc wsh_spawn(const char *wsh_path) {
    int in_pipe[2];  /* in_pipe[1] (parent) -> in_pipe[0] (wsh stdin) */
    int out_pipe[2]; /* out_pipe[1] (wsh stdout/stderr) -> out_pipe[0] (parent) */

    if (pipe(in_pipe) != 0) die("pipe(stdin)");
    if (pipe(out_pipe) != 0) die("pipe(stdout)");

    pid_t pid = fork();
    if (pid < 0) die("fork");

    if (pid == 0) {
        dup2(in_pipe[0], STDIN_FILENO);
        dup2(out_pipe[1], STDOUT_FILENO);
        dup2(out_pipe[1], STDERR_FILENO);

        close(in_pipe[0]);
        close(in_pipe[1]);
        close(out_pipe[0]);
        close(out_pipe[1]);

        execl(wsh_path, wsh_path, (char *)NULL);
        perror("execl wsh");
        _exit(127);
    }

    close(in_pipe[0]);
    close(out_pipe[1]);

    wsh_proc p;
    p.pid = pid;
    p.stdin_fd = in_pipe[1];
    p.stdout_fd = out_pipe[0];
    return p;
}

int wsh_send(wsh_proc *p, const char *line) {
    size_t len = strlen(line);
    if (write(p->stdin_fd, line, len) != (ssize_t)len) return -1;
    if (write(p->stdin_fd, "\n", 1) != 1) return -1;
    return 0;
}

ssize_t wsh_read_until(wsh_proc *p, const char *marker, char *buf, size_t buflen, int timeout_ms) {
    size_t used = 0;
    long long deadline = now_ms() + timeout_ms;
    buf[0] = '\0';

    while (used + 1 < buflen) {
        long long remaining = deadline - now_ms();
        if (remaining <= 0) return -1;

        fd_set rfds;
        FD_ZERO(&rfds);
        FD_SET(p->stdout_fd, &rfds);
        struct timeval tv;
        tv.tv_sec = remaining / 1000;
        tv.tv_usec = (remaining % 1000) * 1000;

        int r = select(p->stdout_fd + 1, &rfds, NULL, NULL, &tv);
        if (r < 0) {
            if (errno == EINTR) continue;
            return -1;
        }
        if (r == 0) return -1; /* timeout */

        ssize_t n = read(p->stdout_fd, buf + used, buflen - 1 - used);
        if (n <= 0) return -1; /* EOF or error */
        used += (size_t)n;
        buf[used] = '\0';

        if (strstr(buf, marker) != NULL) {
            return (ssize_t)used;
        }
    }

    return (ssize_t)used; /* filled the buffer without seeing the marker */
}

int wsh_wait(wsh_proc *p, int timeout_ms) {
    close(p->stdin_fd);

    long long deadline = now_ms() + timeout_ms;
    int status = 0;
    for (;;) {
        pid_t r = waitpid(p->pid, &status, WNOHANG);
        if (r == p->pid) {
            close(p->stdout_fd);
            return status;
        }
        if (r < 0) return -1;
        if (now_ms() >= deadline) return -1;

        struct timespec poll_delay = {.tv_sec = 0, .tv_nsec = 10 * 1000 * 1000};
        nanosleep(&poll_delay, NULL);
    }
}

void wsh_kill(wsh_proc *p) {
    kill(p->pid, SIGKILL);
    int status;
    waitpid(p->pid, &status, 0);
    close(p->stdin_fd);
    close(p->stdout_fd);
}

void wsh_assert_contains(const char *haystack, const char *needle, const char *what) {
    if (strstr(haystack, needle) == NULL) {
        fprintf(stderr, "assertion failed: expected %s to contain %s\n", what, needle);
        fprintf(stderr, "--- actual output ---\n%s\n---------------------\n", haystack);
        exit(1);
    }
}
