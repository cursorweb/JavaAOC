package AOC2022;

import Util.ReadFile;

import java.util.ArrayList;

public class AOC1 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 1);
        String[] lines = file.split("\\n\\n");

        ArrayList<Integer> cals = new ArrayList<>();

        for (String line : lines) {
            String[] nums = line.split("\\n");

            int total = 0;
            for (String num : nums) {
                int n = Integer.parseInt(num);
                total += n;
            }

            cals.add(total);
        }

        cals.sort((a, b) -> -a.compareTo(b));

        System.out.println("Part1: " + cals.get(0));

        int sum = 0;
        for (int i = 0; i < 3; i++) {
            sum += cals.get(i);
        }
        System.out.println("Part2: " + sum);
    }
}
