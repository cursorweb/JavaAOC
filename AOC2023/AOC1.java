package AOC2023;

import util.ReadFile;

public class AOC1 {
    public static void main(String[] args) {
        String file = ReadFile.from(2023, 1);
        String[] lines = file.split("\n");

        int sum = 0;

        for (String line : lines) {
            int firstNumber = -1;
            int lastNumber = -1;

            for (char c : line.toCharArray()) {
                // 0 1 2 3 ... 9
                if ('0' <= c && c <= '9') {
                    int n = c - '0';
                    if (firstNumber == -1) {
                        firstNumber = n;
                    }

                    lastNumber = n;
                }
            }

            // 12 == 1 * 10 + 2 == 12
            sum += firstNumber * 10 + lastNumber;
        }

        System.out.println("Part1: " + sum);

        sum = 0;

        // array indices start at 0
        String[] nums = "zero, one, two, three, four, five, six, seven, eight, nine".split(", ");

        for (String line : lines) {
            int firstNumber = -1;
            int firstNumberIndex = -1;
            int lastNumber = -1;
            int lastNumberIndex = -1;

            for (int i = 0; i < line.length(); i++) {
                char c = line.charAt(i);
                // 0 1 2 3 ... 9
                if ('0' <= c && c <= '9') {
                    int n = c - '0';
                    if (firstNumber == -1) {
                        firstNumber = n;
                        firstNumberIndex = i;
                    }

                    lastNumber = n;
                    lastNumberIndex = i;
                }
            }

            for (int i = 0; i < nums.length; i++) {
                String num = nums[i];
                int firstIndex = line.indexOf(num);

                if (firstIndex == -1) {
                    continue;
                }

                int lastIndex = line.lastIndexOf(num);

                if (firstNumberIndex == -1 || firstIndex < firstNumberIndex) {
                    firstNumberIndex = firstIndex;
                    firstNumber = i;
                }

                if (lastNumberIndex == -1 || lastIndex > lastNumberIndex) {
                    lastNumberIndex = lastIndex;
                    lastNumber = i;
                }
            }

            sum += firstNumber * 10 + lastNumber;
        }

        System.out.println("Part2: " + sum);
    }
}
