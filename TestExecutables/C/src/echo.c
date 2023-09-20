#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>

int main(int argc, char *argv[]) {
    while(1) {
        char *line = NULL;
        size_t n = 0;
        ssize_t result = getline(&line, &n, stdin);
        //printf("result = %zd, n = %zu, line = \"%s\"\n", result, n, line);
        // remove the trailing newline
        line[result-1] = '\0';
        printf("%s", line);
        free(line);
    }
}