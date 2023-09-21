#if !defined(TYPES_H)
#define TYPES_H
#include <sys/types.h>
#include <stdbool.h>
// Enums
typedef enum ExeState {STATEFULL, STATELESS, STATEERROR} ExeState;
typedef enum ExeLang {BASH, BINARY, JAVASCRIPT, TYPESCRIPT, PYTHON, LANGERROR} ExeLang;
// Structs
typedef struct popen2 {
    pid_t pid;
    int   out, in;
} PInfo;
#endif // TYPES_H