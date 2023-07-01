from read import read
file = read()

x = sorted(
    map(
        lambda x: sum(map(
            lambda x: int(x),
            x.split("\n")
        )),
        file.split("\n\n")
    ),
    reverse=True
)

print('part1:', x[0])
print('part2:', sum(x[:3]))
