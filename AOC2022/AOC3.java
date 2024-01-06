package AOC2022;

import util.ReadFile;

public class AOC3 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 3);
        String[] lines = file.split("\\n");

        int part1 = 0;
        String map = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

        for (String line : lines) {
            String left = line.substring(0, line.length() / 2);
            String right = line.substring(line.length() / 2);

            for (char l : left.toCharArray()) {
                if (right.indexOf(l) > -1) {
                    part1 += map.indexOf(l);
                    break;
                }
            }
        }

        int part2 = 0;

        for (int i = 0; i < lines.length; i += 3) {
            String p1 = lines[i];
            String p2 = lines[i + 1];
            String p3 = lines[i + 2];

            for (char p : p1.toCharArray()) {
                if (p2.indexOf(p) > -1 && p3.indexOf(p) > -1) {
                    part2 += map.indexOf(p);
                    break;
                }
            }
        }

        System.out.println("Part1: " + part1);
        System.out.println("Part2: " + part2);
    }
}
