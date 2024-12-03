const fs = require("fs");
const data = fs.readFileSync('x.txt', 'utf8');

{
    const x = [...(data).matchAll(/(?:mul\((\d+),(\d+)\))+?/g)];

    let sum = 0;

    for (const [_, l, r] of x) {
        sum += +l * +r;
    }

    console.log('part1', sum);
}

const x = [...(data).matchAll(/(?:mul\((\d+),(\d+)\))+?|(do\(\))|(don't\(\))/g)];
// console.log(x);

let sum = 0;
let shouldDo = true;

for (const [d, l, r] of x) {
    if (d == "do()") {
        shouldDo = true;
    } else if (d == "don't()") {
        shouldDo = false;
    } else if (shouldDo) {
        sum += +l * +r;
    }
}

console.log('part2', sum);