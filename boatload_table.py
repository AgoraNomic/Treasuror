#!/usr/bin/python3

from sys import argv, exit
from math import ceil

# first arg  = Unit of Flotation
# second arg = number of rows

if len(argv) == 3:
    uf   = float(argv[1])
    rows = int(argv[2])
    for n in range(1, rows+1):
        print(f"{n:>3} boatloads = {ceil(n*uf):>3} assets")
else:
    print("needs two args!")
    exit(1);
