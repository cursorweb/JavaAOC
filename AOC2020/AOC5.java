package AOC2020;

import util.ReadFile;

import java.util.ArrayList;
import java.util.List;

public class AOC5 {
    public static void main(String[] args) {
        String f = ReadFile.from(2020, 5);
        String[] lines = f.split("\\n");

        ArrayList<Integer> nums = new ArrayList<>();

        for (String line : lines) {
            int lowerRow = 0;
            int upperRow = 127;

            int lowerCol = 0;
            int upperCol = 7;

            for (char c : line.toCharArray()) {
                if (c == 'F') {
                    // keep lower half
                    upperRow = lowerRow + (upperRow - lowerRow) / 2;
                }
                if (c == 'B') {
                    // keep upper half
                    lowerRow = lowerRow + (upperRow + 1 - lowerRow) / 2;
                }
                if (c == 'R') {
                    // keep upper half
                    lowerCol = lowerCol + (upperCol + 1 - lowerCol) / 2;
                }
                if (c == 'L') {
                    upperCol = lowerCol + (upperCol - lowerCol) / 2;
                }
            }

            nums.add(lowerRow * 8 + lowerCol);
        }

        nums.sort((a, b) -> -a.compareTo(b));

        System.out.println("Part1: " + nums.get(0));

        List<Integer> ns = nums.subList(1, nums.size() - 1);
        Integer prev = nums.get(0);
        for (Integer i : ns) {
            if (prev - i != 1) {
                // descending
                System.out.println("Part2: " + (prev - 1));
            }
            prev = i;
        }
    }
}
