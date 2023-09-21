#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>

int main(int argc, char *argv[]) {
    while(1) {
        char *line = NULL;
        size_t n = 0;
        ssize_t result = getline(&line, &n, stdin);
        printf("%s", line);
        free(line);
    }
}