import sys
import os
import pathlib

def read():
    namespace = sys._getframe(1).f_globals
    folder = os.path.dirname(namespace['__file__'])
    path = pathlib.Path(folder + '/data.txt').resolve()

    file = open(path, 'r')

    return ''.join(file.readlines())