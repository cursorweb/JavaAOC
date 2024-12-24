import math

with open("src/aoc2024/day11/data.txt") as f:
    nums = [int(x) for x in f.read().split(" ")]

nums_temp = nums.copy()


def count_digits(x: int):
    return int(math.log10(x) + 1)


def split_digits(x: int):
    digs = count_digits(x) / 2
    left = x // (10**digs)
    right = x % (10**digs)
    return int(left), int(right)


cache = {}


def count_times(num, it=75):
    if it == 0:
        return 1
    if (num, it) in cache:
        return cache[num, it]

    if num == 0:
        out = count_times(1, it - 1)
    elif count_digits(num) % 2 == 0:
        l, r = split_digits(num)
        out = count_times(l, it - 1) + count_times(r, it - 1)
    else:
        out = count_times(num * 2024, it - 1)

    cache[num, it] = out
    return out


part1 = sum([count_times(n, 25) for n in nums])
part2 = sum([count_times(n) for n in nums])

print("part1:", part1)
print("part2:", part2)
