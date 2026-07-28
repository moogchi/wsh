#define _POSIX_C_SOURCE 200809L

#include "common.h"

#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>

int main(void) {
    const char *wsh_path = getenv("WSH_BIN");
    if (!wsh_path) {
        fprintf(stderr, "WSH_BIN not set\n");
        return 1;
    }

    wsh_proc p = wsh_spawn(wsh_path);
    char buf[4096];

    if (wsh_read_until(&p, "» ", buf, sizeof(buf), 5000) < 0) {
        fprintf(stderr, "timed out waiting for initial prompt\n");
        wsh_kill(&p);
        return 1;
    }

    if (wsh_send(&p, "echo hello wsh") != 0) {
        fprintf(stderr, "failed to send echo command\n");
        wsh_kill(&p);
        return 1;
    }

    if (wsh_read_until(&p, "» ", buf, sizeof(buf), 5000) < 0) {
        fprintf(stderr, "timed out waiting for echo output\n");
        wsh_kill(&p);
        return 1;
    }
    wsh_assert_contains(buf, "hello wsh", "echo output");

    wsh_send(&p, "exit");
    int status = wsh_wait(&p, 5000);
    if (status < 0) {
        fprintf(stderr, "wsh did not exit in time\n");
        wsh_kill(&p);
        return 1;
    }
    if (!WIFEXITED(status) || WEXITSTATUS(status) != 0) {
        fprintf(stderr, "wsh exited abnormally (raw status=%d)\n", status);
        return 1;
    }

    printf("OK\n");
    return 0;
}
