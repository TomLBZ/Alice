#include <string.h>
#include <time.h>
#include "./globals.h"
#include "./loop.h"
#include "./popen2.h"

void loop() {
    while(1) {
        char* input = getInputUnsafe(stdin);
        int processIndex = getAvailableProcessIndex();
        PInfo process = pInfoList[processIndex];
        if(process.pid == -1) process = startProcess();
        if(process.pid != -1) {
            processCount++;
            if(exeState == STATEFULL){
                int stateId = getStateId();
                // resize the stateIdList
                stateIdList = realloc(stateIdList, processCount * sizeof(int));
                stateIdList[processIndex] = stateId;
            }
            // resize the availabilityList
            availabilityList = realloc(availabilityList, processCount * sizeof(int));
            availabilityList[processIndex] = 1;
            // sends text to the process stdin
            fprintf((FILE* restrict)process.in, "%s\n", input);
            // and get the process stdout
            char* output = getInputUnsafe((FILE* restrict)process.out);
            printf("%s\n", output);
            availabilityList[processIndex] = 0;
        }
        free(input);
    }
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
    PInfo pInfo = duplicate(POPEN2_NULL);
    if(exeLang == BASH) {
        int totalStrLen = strlen("bash -c ") + strlen(exePath) + 1;
        char* cmdline = malloc(totalStrLen);
        strcpy(cmdline, "bash -c ");
        strcat(cmdline, exePath);
        if(popen2(cmdline, &pInfo)) return pInfo;
        return POPEN2_NULL;
    } else if(exeLang == BINARY) {
        if(popen2(exePath, &pInfo)) return pInfo;
        return POPEN2_NULL;
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