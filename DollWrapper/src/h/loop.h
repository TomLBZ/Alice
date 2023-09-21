#if !defined(LOOP_H)
#define LOOP_H
#include "types.h"
void loop();
char* getInputUnsafe(FILE* stream);
ssize_t getAvailableProcessIndex();
PInfo duplicate(PInfo pInfo);
PInfo startProcess();
size_t getStateId();
char* getResponse(char* input, int fdin, int fdout);
bool isAlive(int pid);
#endif