#if !defined(POPEN2_H)
#define POPEN2_H
#include "./types.h"
int popen2(const char *cmdline, PInfo *info);
#endif