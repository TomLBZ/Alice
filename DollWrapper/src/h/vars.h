#if !defined(VARS_H)
#define VARS_H
#include "types.h"
// Consts
extern const PInfo POPEN2_NULL;
// Vars
extern int exeMaxInstances;
extern char* exePath;
extern ExeState exeState;
extern ExeLang exeLang;
extern PInfo* pInfoList;
extern int processCount;
extern bool* availabilityList;
extern int* stateIdList;

#endif // VARS_H
