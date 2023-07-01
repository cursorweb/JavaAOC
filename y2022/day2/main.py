"""
A X = 1 Rock
B Y = 2 Paper ^
C Z = 3 Scissors ^
"""

from read import read
file = read()

"""
0 = Lose
3 = Draw
6 = Win
"""


def get_winner(them: str, you: str) -> int:
    if them == 'A' and you == 'X' or them == 'B' and you == 'Y' or them == 'C' and you == 'Z':
        return 3

    if them == 'A' and you == 'Z' or them == 'B' and you == 'X' or them == 'C' and you == 'Y':
        return 0

    return 6


def get_score(them: str, you: str) -> int:
    score = 0
    match you:
        case 'X':
            if them == 'A':
                score = 3
            if them == 'B':
                score = 1
            if them == 'C':
                score = 2
        case 'Y':
            score = 3 + get_lvalue(them)
        case 'Z':
            score = 6
            if them == 'A':
                score += 2
            if them == 'B':
                score += 3
            if them == 'C':
                score += 1

    return score


def get_value(x: str):
    return ord(x) - ord('W')  # W = 0 X = 1 ...


def get_lvalue(x: str):
    return ord(x) - ord('@')  # @ = 0


score1 = 0
score2 = 0

for line in file:
    them, you = line.split(' ')
    score1 += get_winner(them, you) + get_value(you)

    score2 += get_score(them, you)

print('part1:', score1)
print('part2:', score2)
