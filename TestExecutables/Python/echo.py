# This is a simple python echo program that prints out the command line arguments
# it receives.  It is used to test the Python interpreter.

import sys

for arg in sys.argv:
    print(arg + " ", end="")

# print "-- Doll: echo.py: end of program --\n"
print("\n-- Doll: echo.py: end of program --\n")

sys.exit(0)