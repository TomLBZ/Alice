#if !defined(LOOP_H)
#define LOOP_H
#include "./types.h"
void loop();
char* getInputUnsafe(FILE* restrict stream);
int getAvailableProcessIndex();
PInfo duplicate(PInfo pInfo);
PInfo startProcess();
int getStateId();
void writeFd(int fd, char* str);
char* readFd(int fd);
#endif