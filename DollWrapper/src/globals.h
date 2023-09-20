#include <stdio.h>
#include <stdlib.h>
#include "./types.h"
#if !defined(GLOBALS_H)
#define GLOBALS_H
// Consts
extern const PInfo POPEN2_NULL;
// Vars
extern char* exePath;
extern ExeState exeState;
extern ExeLang exeLang;
extern PInfo* pInfoList;
extern int processCount;
extern int* availabilityList;
extern int* stateIdList;
#endif // GLOBALS_H