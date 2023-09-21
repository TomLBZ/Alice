#include <time.h>
#include <unistd.h>
#include <signal.h>
#include "h/globals.h"
#include "h/loop.h"
#include "h/popen2.h"

void loop() {
    printf("=== loop started ===\n");
    size_t loopCount = 0;
    while(true) {
        loopCount++;
        printf(">>> loop number: %ld; instances: %ld\n", loopCount, processCount);
        char* input = getInputUnsafe(stdin);
        ssize_t processIndex = getAvailableProcessIndex();
        PInfo process = POPEN2_NULL;
        if (processIndex != -1) process = pInfoList[processIndex];
        if(process.pid == -1) {
            if (processCount < exeMaxInstances) process = startProcess();
            else {
                printf("error: max instances (%ld) reached\n", exeMaxInstances);
                free(input);
                continue;
            }
        }
        if(process.pid != -1) {
            processIndex = (ssize_t)processCount;
            printf("sending to index: %ld; pid: %d; in: %d, out: %d\n", processIndex, process.pid, process.in, process.out);
            pInfoList[processIndex] = process;
            if(exeState == STATEFULL){
                size_t stateId = getStateId();
                printf("started instance using stateId: %ld\n", stateId);
                stateIdList[processIndex] = stateId;
            }
            availabilityList[processIndex] = false;
            processCount++;
            char* output = getResponse(input, process.in, process.out);
            availabilityList[processIndex] = true;
            // if the pid is not alive, means the program crashed. so, remove it from the lists
            if(!isAlive(process.pid)) {
                pInfoList[processIndex] = POPEN2_NULL;
                availabilityList[processIndex] = false;
                if(exeState == STATEFULL) stateIdList[processIndex] = 0;
                processCount--;
            }
            printf("output: %s\n", output);
            free(output);
        } else {
            printf("error: process could not be started\n");
        }
        free(input);
    }
    printf("=== loop stopped ===\n");
}

char* getInputUnsafe(FILE* stream) {
    char *line = NULL;
    size_t n = 0;
    ssize_t result = getline(&line, &n, stream);
    return line;
}

ssize_t getAvailableProcessIndex() {
    if(processCount == 0) return -1;
    for(ssize_t i = 0; i < (ssize_t)processCount; i++) {
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

size_t getStateId() {
    srand((unsigned int)time(NULL));
    return (size_t)rand();
}

char* getResponse(char* input, int fdin, int fdout) {
    // dup fdin and fdout
    //int fdin2 = dup(fdin);
    //int fdout2 = dup(fdout);
    // write to fin
    // FILE* restrict fin = fdopen(fdin2, "w");
    // fprintf(fin, "%s\n", input);
    // fclose(fin);
    // printf("wrote to fd: %d\n", fdin);
    // // read from fout
    // FILE* restrict fout = fdopen(fdout2, "r");
    // char* str = getInputUnsafe(fout);
    // fclose(fout);
    write(fdin, input, strlen(input));
    //close(fdin);
    printf("wrote to fd: %d\n", fdin);


    // start a new thread to read from fdout
    

    char buf[100];
    read(fdout, buf, strlen(buf));
    printf("%s\n", buf);
    char* output = (char*)malloc(sizeof(char) * (strlen(buf) + 1));
    strcpy(output, buf);
    return output;
}

bool isAlive(int pid) {
    return kill(pid, 0) == 0;
}