import os
import sys
from pymalloc import pymalloc as malloc, pyfree as free

sys.path.append(os.path.dirname(__file__))

# If malloc() is captured, so is free() etc, so less important to test those.
def main():
    malloc(50 * 1024 * 1024)


main()
