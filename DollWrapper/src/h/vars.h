#if !defined(VARS_H)
#define VARS_H
#include "types.h"
// Consts
extern const PInfo POPEN2_NULL;
// Vars
extern char* exePath;
extern ExeState exeState;
extern ExeLang exeLang;
extern size_t exeMaxInstances;
extern size_t processCount;
extern PInfo* pInfoList;
extern size_t* stateIdList;
extern bool* availabilityList;

#endif // VARS_H
