#include "h/globals.h"
#include "h/setup.h"
#include "h/loop.h"
#include "h/main.h"

// defines global constants
const PInfo POPEN2_NULL = {-1, -1, -1};
int exeMaxInstances = 0;
char* exePath = NULL;
ExeState exeState = STATEERROR;
ExeLang exeLang = LANGERROR;
PInfo* pInfoList = NULL;
int processCount = 0;
bool* availabilityList = NULL;
int* stateIdList = NULL;

int main(int argc, char *argv[]) {
    if (!setup()) return -1;
    loop();
    cleanup();
}

void cleanup() {
    printf("=== cleanup started ===\n");
    free(exePath);
    free(pInfoList);
    free(availabilityList);
    free(stateIdList);
    printf("=== cleanup done ===\n");
}