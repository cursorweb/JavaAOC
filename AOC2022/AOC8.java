package AOC2022;

import util.ReadFile;

public class AOC8 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 8);
        String[] lines = file.split("\n");

        int gridWidth = lines[0].length();
        int gridHeight = lines.length;

        int[][] grid = new int[gridHeight][gridWidth];

        for (int i = 0; i < lines.length; i++) {
            String line = lines[i];
            char[] chars = line.toCharArray();

            for (int j = 0; j < chars.length; j++) {
                grid[i][j] = chars[j] - '0';
            }
        }

        int visibles = 2 * gridWidth + 2 * gridHeight - 4;
        int maxScenic = 0;

        for (int y = 1; y < gridHeight - 1; y++) {
            for (int x = 1; x < gridWidth - 1; x++) {
                int horizVisibles = 2;
                int vertVisibles = 2;

                int height = grid[y][x];

                int right = 0;
                int left = 0;
                int top = 0;
                int bottom = 0;

                // right
                for (int i = x + 1; i < gridWidth; i++) {
                    right++;
                    if (grid[y][i] >= height) {
                        horizVisibles--;
                        break;
                    }
                }

                // left
                for (int i = x - 1; i >= 0; i--) {
                    left++;
                    if (grid[y][i] >= height) {
                        horizVisibles--;
                        break;
                    }
                }

                // top
                for (int i = y + 1; i < gridHeight; i++) {
                    top++;
                    if (grid[i][x] >= height) {
                        vertVisibles--;
                        break;
                    }
                }

                // bottom
                for (int i = y - 1; i >= 0; i--) {
                    bottom++;
                    if (grid[i][x] >= height) {
                        vertVisibles--;
                        break;
                    }
                }

                if (horizVisibles > 0 || vertVisibles > 0) {
                    visibles++;
                }

                int score = left * top * bottom * right;
                if (score > maxScenic) {
                    maxScenic = score;
                }
            }
        }

        System.out.println("Part1: " + visibles);
        System.out.println("Part2: " + maxScenic);
    }
}
