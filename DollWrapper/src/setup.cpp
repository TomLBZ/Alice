#include "h/globals.h"
#include "h/setup.h"

char* getExePath() {
    char* exepath = getenv("DOLLEXEPATH");
    char* exename = getenv("DOLLEXENAME");
    if (exepath == NULL || exename == NULL) {
        printf("Error: environment variables DOLLEXEPATH and/or DOLLEXENAME not set properly");
        return NULL;
    }
    bool isEndsWithSlash = exepath[strlen(exepath)-1] == '/';
    size_t totalStrLen = strlen(exepath) + strlen(exename) + 1 + (isEndsWithSlash ? 0 : 1);
    char* output = (char*)malloc(sizeof(char) * totalStrLen);
    strcpy(output, exepath);
    if (isEndsWithSlash) {
        strcat(output, exename);
    } else {
        strcat(output, "/");
        strcat(output, exename);
    }
    return output;
}

ExeState getExeState() {
    char* exestate = getenv("DOLLEXESTATE");
    if (exestate == NULL) {
        printf("Error: environment variable DOLLEXESTATE not set properly");
        return STATEERROR;
    }
    if (strcmp(exestate, "0") != 0 && strcmp(exestate, "1") != 0) {
        printf("Error: environment variable DOLLEXESTATE not set properly");
        return STATEERROR;
    }
    return strcmp(exestate, "0") == 0 ? STATELESS : STATEFULL;
}

ExeLang getExeLang() {
    char* exelang = getenv("DOLLEXELANG");
    if (exelang == NULL) {
        printf("Error: environment variable DOLLEXELANG not set properly");
        return LANGERROR;
    }
    // exelang can only be "sh", "c", "cs", "js", "ts", "py", "rs", otherwise exit with error
    if (strcmp(exelang, "sh") == 0) {
        return BASH;
    } else if (strcmp(exelang, "c") == 0) {
        return BINARY;
    } else if (strcmp(exelang, "cs") == 0) {
        return BINARY;
    } else if (strcmp(exelang, "js") == 0) {
        return JAVASCRIPT;
    } else if (strcmp(exelang, "ts") == 0) {
        return TYPESCRIPT;
    } else if (strcmp(exelang, "py") == 0) {
        return PYTHON;
    } else if (strcmp(exelang, "rs") == 0) {
        return BINARY;
    } else {
        printf("Error: environment variable DOLLEXELANG not set properly");
        return LANGERROR;
    }
}

size_t getExeMaxInstances() {
    char* exemaxinstances = getenv("DOLLEXEMAXINSTANCES");
    if (exemaxinstances == NULL) {
        printf("Error: environment variable DOLLEXEMAXINSTANCES not set properly");
        return 0;
    }
    return (size_t)atol(exemaxinstances);
}

bool setup(){
    printf("=== setup started ===\n");
    // get env vars
    // max instances is used to init the arrays, so must be set first and be valid
    exeMaxInstances = getExeMaxInstances();
    if (exeMaxInstances == 0) return false;
    pInfoList = (PInfo*)malloc(sizeof(PInfo) * exeMaxInstances);
    availabilityList = (bool*)malloc(sizeof(bool) * exeMaxInstances);
    stateIdList = (size_t*)malloc(sizeof(size_t) * exeMaxInstances);
    if (pInfoList == NULL || availabilityList == NULL || stateIdList == NULL) {
        printf("Error: malloc failed");
        return false;
    }
    printf("exeMaxInstances: %ld\n", exeMaxInstances);
    // exePath, exeState, exeLang are used to start the processes, so must be set after
    exePath = getExePath();
    if (exePath == NULL) return false;
    printf("exePath: %s\n", exePath);
    exeState = getExeState();
    if (exeState == STATEERROR) return false;
    printf("exeState: %d\n", exeState);
    exeLang = getExeLang();
    if (exeLang == LANGERROR) return false;
    printf("exeLang: %d\n", exeLang);
    printf("=== setup finished ===\n");
    return true;
}