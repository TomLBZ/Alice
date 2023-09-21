#include <time.h>
#include <unistd.h>
#include "h/globals.h"
#include "h/loop.h"
#include "h/popen2.h"

void loop() {
    printf("=== loop started ===\n");
    long long int loopCount = 0;
    while(true) {
        loopCount++;
        printf(">>> loop number: %lld\n", loopCount);
        char* input = getInputUnsafe(stdin);
        int processIndex = getAvailableProcessIndex();
        PInfo process = POPEN2_NULL;
        if (processIndex != -1) process = pInfoList[processIndex];
        if(process.pid == -1) {
            if (processCount < exeMaxInstances) process = startProcess();
            else {
                printf("error: max instances (%d) reached\n", exeMaxInstances);
                free(input);
                continue;
            }
        }
        if(process.pid != -1) {
            processIndex = processCount;
            printf("sending to index: %d; pid: %d; in: %d, out: %d\n", processIndex, process.pid, process.in, process.out);
            pInfoList[processIndex] = process;
            if(exeState == STATEFULL){
                int stateId = getStateId();
                printf("started instance using stateId: %d\n", stateId);
                stateIdList[processIndex] = stateId;
            }
            availabilityList[processIndex] = false;
            processCount++;
            // sends text to the process stdin
            writeFd(process.in, input);
            // and get the process stdout
            char* output = readFd(process.out);
            // write(process.in, input, strlen(input));
            // close(process.in);
            // char output[100];
            // read(process.out, output, 100);
            // printf("%s\n", output);
            availabilityList[processIndex] = true;
        } else {
            printf("error: process could not be started\n");
        }
        free(input);
    }
    printf("=== loop stopped ===\n");
}

char* getInputUnsafe(FILE* restrict stream) {
    char *line = NULL;
    size_t n = 0;
    ssize_t result = getline(&line, &n, stream);
    line[result-1] = '\0';
    return line;
}

int getAvailableProcessIndex() {
    if(processCount == 0) return -1;
    for(int i = 0; i < processCount; i++) {
        if(availabilityList[i] == 0) { // available
            if(exeState == STATELESS) return i;
            // stateIdList[i] will be the id of the session. sessions are not implemented yet,
            // so pid will be returned as long as the stateIdList[i] > 0
            else if(stateIdList[i] > 0) return i;
        }
    }
    return -1;
}

PInfo duplicate(PInfo pInfo) { // pass by value
    return pInfo;
}

PInfo startProcess() {
    if(exeLang == BASH || exeLang == BINARY) {
        return popen2(exePath);
    } else if(exeLang == JAVASCRIPT) {
        return POPEN2_NULL;
    } else if(exeLang == TYPESCRIPT) {
        return POPEN2_NULL;
    } else if(exeLang == PYTHON) {
        return POPEN2_NULL;
    } else {
        return POPEN2_NULL;
    }
}

int getStateId() {
    srand((unsigned int)time(NULL));
    return rand();
}

void writeFd(int fd, char* str) {
    int fd2 = dup(fd);
    FILE* restrict f = fdopen(fd2, "w");
    fprintf(f, "%s\n", str);
    fclose(f);
    printf("wrote to fd: %d\n", fd);
}

char* readFd(int fd) {
    int fd2 = dup(fd);
    FILE* restrict f = fdopen(fd2, "r");
    char* str = getInputUnsafe(f);
    fclose(f);
    return str;
}