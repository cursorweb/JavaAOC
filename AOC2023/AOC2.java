package AOC2023;

import Util.ReadFile;

public class AOC2 {
    public static void main(String[] args) {
        String file = ReadFile.from(2023, 2);
        String[] lines = file.split("\n");

        String[] names = { "red", "green", "blue" };

        int validGames = 0;
        int sum = 0;

        for (int i = 0; i < lines.length; i++) {
            boolean isValid = true;
            int[] max = { 0, 0, 0 };

            String line = lines[i];
            String rest = line.split(": ")[1];
            String[] revealedCubes = rest.split("; ");

            // 3 blue, 4 green, 5 red
            for (String revealed : revealedCubes) {
                // "3 blue",  "4 green",  "5 red"
                String[] cubes = revealed.split(", ");

                for (String cube : cubes) {
                    String[] split = cube.split(" ");
                    int count = Integer.parseInt(split[0]);
                    String cubeType = split[1];

                    for (int j = 0; j < names.length; j++) {
                        if (cubeType.equals(names[j])) {
                            max[j] = Math.max(max[j], count);

                            if (count > 12 + j) {
                                isValid = false;
                            }
                        }
                    }
                }
            }

            if (isValid) {
                validGames += i + 1;
            }

            sum += max[0] * max[1] * max[2];
        }

        System.out.println("Part1: " + validGames);
        System.out.println("Part2: " + sum);
    }
}
