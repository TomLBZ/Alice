#if !defined(SETUP_H)
#define SETUP_H
#include "types.h"
bool setup();
char* getExePath();
ExeState getExeState();
ExeLang getExeLang();
int getExeMaxInstances();
#endif