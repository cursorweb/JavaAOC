import importlib
import sys

# change this!
year = 2022
day = 3

importlib.import_module(f'y{year}.day{day if len(sys.argv) == 1 else int(sys.argv[1]) }.main')