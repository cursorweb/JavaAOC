import re
from read import read

file = read()

names = "zero, one, two, three, four, five, six, seven, eight, nine".split(", ")

total_part1 = 0
total_part2 = 0
for line in file:
    digits_found = re.findall("\\d", line)
    total_part1 += int(digits_found[0] + digits_found[-1])

    numbers_found: list[str] = re.findall(f"(?=({'|'.join(names)}|\\d))", line)
    num1 = (
        int(numbers_found[0])
        if numbers_found[0].isnumeric()
        else names.index(numbers_found[0])
    )
    num2 = (
        int(numbers_found[-1])
        if numbers_found[-1].isnumeric()
        else names.index(numbers_found[-1])
    )

    total_part2 += num1 * 10 + num2


print("Part1:", total_part1)
print("Part2:", total_part2)
