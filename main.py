import importlib
import sys

# change this!
year = 2024
day = 11

importlib.import_module(
    f"aoc{year}.day{day if len(sys.argv) == 1 else int(sys.argv[1]) }.main"
)
