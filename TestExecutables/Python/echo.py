#!/usr/bin/python3

import sys
while(True):
    # detect ctrl + c
    try:
        input = sys.stdin.readline()
        print(input, end='')
    except KeyboardInterrupt:
        break