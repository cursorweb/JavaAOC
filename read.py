import sys
import os
import pathlib
from typing import overload, Literal

@overload
def read(as_lines: Literal[True] = ...) -> list[str]: ...
@overload
def read(as_lines: Literal[False]) -> str: ...

def read(as_lines=True) -> list[str] | str:
    namespace = sys._getframe(1).f_globals
    folder = os.path.dirname(namespace['__file__'])
    path = pathlib.Path(folder + '/data.txt').resolve()

    file = open(path, 'r')

    return list(map(lambda s : s.rstrip('\n'), file.readlines())) if as_lines else ''.join(file.readlines())