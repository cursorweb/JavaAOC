from read import read
from functools import reduce
file = read()


def get_val(c: str) -> int:
    match c:
        case c if 'a' <= c <= 'z':
            return ord(c) - ord('a') + 1

        case c if 'A' <= c <= 'Z':
            return ord(c) - ord('A') + 27

        case _: raise Exception("unreachable state: " + c)


def part1(line: str) -> int:
    hlf = len(line) // 2
    first = set(line[:hlf])
    second = set(line[hlf:])

    c = get_val(first.intersection(second).pop())
    return c


print('part1:', sum([part1(line) for line in file]))
part2 = 0
for i in range(0, len(file), 3):
    line = map(set, file[i:i + 3])
    res = reduce(lambda prev, curr: prev.intersection(curr), line).pop()
    part2 += get_val(res)
print('part2:', part2)