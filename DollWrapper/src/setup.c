#include <string.h>
#include "./globals.h"
#include "./setup.h"

void getExePath() {
    char* exepath = getenv("EXEPATH");
    char* exename = getenv("EXENAME");
    if (exepath == NULL || exename == NULL) {
        printf("Error: environment variables EXEPATH and/or EXENAME not set properly");
        exePath = NULL;
        return;
    }
    int totalStrLen = strlen(exepath) + strlen(exename) + 1;
    exepath = realloc(exepath, totalStrLen);
    if (exepath[strlen(exepath)-1] == '/') {
        strcat(exepath, exename);
    } else {
        strcat(exepath, "/");
        strcat(exepath, exename);
    }
    exePath = exepath;
}

void getExeState() {
    char* exestate = getenv("EXESTATE");
    if (exestate == NULL) {
        printf("Error: environment variable EXESTATE not set properly");
        exeState = STATEERROR;
        return;
    }
    if (strcmp(exestate, "0") != 0 && strcmp(exestate, "1") != 0) {
        printf("Error: environment variable EXESTATE not set properly");
        exeState = STATEERROR;
        return;
    }
    exeState = strcmp(exestate, "0") == 0 ? STATELESS : STATEFULL;
}

void getExeLang() {
    char* exelang = getenv("EXELANG");
    if (exelang == NULL) {
        printf("Error: environment variable EXELANG not set properly");
        exeLang = LANGERROR;
        return;
    }
    // exelang can only be "sh", "c", "cs", "js", "ts", "py", "rs", otherwise exit with error
    if (strcmp(exelang, "sh") == 0) {
        exeLang = BASH;
    } else if (strcmp(exelang, "c") == 0) {
        exeLang = BINARY;
    } else if (strcmp(exelang, "cs") == 0) {
        exeLang = BINARY;
    } else if (strcmp(exelang, "js") == 0) {
        exeLang = JAVASCRIPT;
    } else if (strcmp(exelang, "ts") == 0) {
        exeLang = TYPESCRIPT;
    } else if (strcmp(exelang, "py") == 0) {
        exeLang = PYTHON;
    } else if (strcmp(exelang, "rs") == 0) {
        exeLang = BINARY;
    } else {
        printf("Error: environment variable EXELANG not set properly");
        exeLang = LANGERROR;
    }
}

int setup(){
    // get env vars
    getExePath();
    getExeState();
    getExeLang();
    if (exePath == NULL || exeState == STATEERROR || exeLang == LANGERROR) {
        return 1;
    }
}