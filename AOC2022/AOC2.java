package AOC2022;

import Util.ReadFile;

public class AOC2 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 2);
        String[] lines = file.split("\\n");

        int part1sum = 0;
        int part2sum = 0;

        for (String line : lines) {
            int opp = line.charAt(0) - 'A';
            int you = line.charAt(2) - 'X';

            int part1;
            int part2 = 0;

            if (you == opp) {
                part1 = 3 + you + 1;
            } else if (you == 1 && opp == 0 || you == 2 && opp == 1 || you == 0 && opp == 2) {
                part1 = 6 + you + 1;
            } else {
                part1 = you + 1;
            }

            if (you == 0) { // lose
                switch (opp) {
                    case 1:
                        // part2 = 0;
                        break;
                    case 2:
                        part2 = 1;
                        break;
                    case 0:
                        part2 = 2;
                        break;
                }

                part2 += 1;
            } else if (you == 1) { // tie
                part2 = opp + 1 + 3;
            } else if (you == 2) { // win
                if (opp == 0) {
                    part2 = 1;
                } else if (opp == 1) {
                    part2 = 2;
                }
                // part2 = 0;

                part2 += 6 + 1;
            }

            part1sum += part1;
            part2sum += part2;
        }

        System.out.println("Part1: " + part1sum);
        System.out.println("Part2: " + part2sum);
    }
}
