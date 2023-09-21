#include <unistd.h>
#include "h/globals.h"
#include "h/popen2.h"

PInfo popen2(const char *cmdline) {
    pid_t p;
    int pipe_stdin[2], pipe_stdout[2];
    PInfo output = POPEN2_NULL;
    if(pipe(pipe_stdin)) return output;
    if(pipe(pipe_stdout)) return output;
    p = fork();
    if(p < 0) return output; /* Fork failed */
    if(p == 0) { /* child */
        close(pipe_stdin[1]);
        dup2(pipe_stdin[0], 0);
        close(pipe_stdout[0]);
        dup2(pipe_stdout[1], 1);
        execl("/bin/bash", "bash", "-c", cmdline, (char *)0);
        printf("execl failed\n");
        perror("execl"); exit(99);
    }
    output.pid = p;
    output.in = pipe_stdin[1];
    output.out = pipe_stdout[0];
    return output;
}