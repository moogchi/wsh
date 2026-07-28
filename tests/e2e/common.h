#ifndef WSH_TEST_COMMON_H
#define WSH_TEST_COMMON_H

#include <sys/types.h>
#include <stddef.h>

typedef struct {
    pid_t pid;
    int stdin_fd;  /* parent write end -> wsh's stdin */
    int stdout_fd; /* wsh's stdout -> parent read end */
} wsh_proc;

/* Forks and execs the wsh binary at wsh_path with stdin/stdout (and stderr)
 * redirected to pipes owned by the returned handle. Exits the test process
 * on failure to fork/pipe. */
wsh_proc wsh_spawn(const char *wsh_path);

/* Writes `line` followed by a newline to wsh's stdin. Returns 0 on success,
 * -1 on a short/failed write. */
int wsh_send(wsh_proc *p, const char *line);

/* Reads from wsh's stdout, accumulating into buf (always NUL-terminated),
 * until `marker` appears in the accumulated text or timeout_ms elapses.
 * Returns the number of bytes accumulated, or -1 on timeout/EOF/error. */
ssize_t wsh_read_until(wsh_proc *p, const char *marker, char *buf, size_t buflen, int timeout_ms);

/* Closes wsh's stdin (signals EOF) and waits up to timeout_ms for it to
 * exit. Returns the status as reported by waitpid (use WIFEXITED /
 * WEXITSTATUS on it), or -1 if the wait timed out. */
int wsh_wait(wsh_proc *p, int timeout_ms);

/* Force-kills and reaps the process; used for cleanup after a failed
 * assertion so the test binary doesn't leave orphaned children behind. */
void wsh_kill(wsh_proc *p);

/* Prints a diagnostic and exit(1)s the test process if `needle` is not
 * found in `haystack`. `what` is a short human label for the assertion. */
void wsh_assert_contains(const char *haystack, const char *needle, const char *what);

#endif
