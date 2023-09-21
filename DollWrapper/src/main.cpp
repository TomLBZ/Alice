#include "h/globals.h"
#include "h/setup.h"
#include "h/loop.h"
#include "h/main.h"
#include "externals/App.h"

// defines global constants
const PInfo POPEN2_NULL = {-1, -1, -1};
char* exePath = NULL;
ExeState exeState = STATEERROR;
ExeLang exeLang = LANGERROR;
size_t exeMaxInstances = 0;
size_t processCount = 0;
PInfo* pInfoList = NULL;
size_t* stateIdList = NULL;
bool* availabilityList = NULL;

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