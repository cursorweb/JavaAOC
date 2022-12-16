package AOC2022;

import Util.ReadFile;

import java.util.Arrays;

public class AOC12 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 12);
        String[] lines = file.split("\\n");

        int height = lines.length;
        int width = lines[0].length();

        char[][] grid = new char[height][width];

        for (int i = 0; i < height; i++) {
            for (int j = 0; j < width; j++) {
                grid[i][j] = lines[i].charAt(j);
            }
        }

        System.out.println(Arrays.deepToString(grid));
    }

    private static
    class Point {

    }
}
