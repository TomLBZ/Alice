// This is a command line program that prints out its arguments.
// It is used to test the basic functionalities of the Doll wrapper.

#include <stdio.h>

int main(int argc, char *argv[]) {
  int i;
  for (i = 0; i < argc; i++) {
    printf("%s ", argv[i]);
  }
  printf("\n");
  // print "-- Doll: echo.c: end of program --\n"
  printf("-- Doll: echo.c: end of program --\n");
  return 0;
}