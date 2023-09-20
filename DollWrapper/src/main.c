#include "./globals.h"
//#include "./main.h"
#include "./loop.h"
#include "./setup.h"

// defines global constants
const PInfo POPEN2_NULL = {-1, -1, -1};
char* exePath;
ExeState exeState;
ExeLang exeLang;
PInfo* pInfoList;
int processCount;
int* availabilityList;
int* stateIdList;

int main(int argc, char *argv[]) {
    if (!setup()) return -1;
    loop();
}