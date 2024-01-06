package AOC2022;

import util.ReadFile;

public class AOC4 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 4);
        String[] lines = file.split("\\n");

        int part1 = 0;
        int part2 = 0;

        for (String line : lines) {
            String[] rangesStr = line.split(",");
            String[] range1Str = rangesStr[0].split("-");
            String[] range2Str = rangesStr[1].split("-");

            Range range1 = new Range(Integer.parseInt(range1Str[0]), Integer.parseInt(range1Str[1]));
            Range range2 = new Range(Integer.parseInt(range2Str[0]), Integer.parseInt(range2Str[1]));

            if (range1.fullyContain(range2)) {
                part1++;
            }

            if (range1.overlap(range2)) {
                part2++;
            }
        }

        System.out.println("Part1: " + part1);
        System.out.println("Part2: " + part2);
    }

    private static class Range {
        private final int start;
        private final int end;

        public Range(int start, int end) {
            this.start = start;
            this.end = end;
        }

        public boolean fullyContain(Range other) {
            return (this.start >= other.start && this.end <= other.end) || (other.start >= this.start && other.end <= this.end);
        }

        /*       |....|
         *    |....|
         *
         *    |....|
         *       |....|
         */
        public boolean overlap(Range other) {
            // alternate: this.start <= other.end && this.end >= other.start
            return fullyContain(other) || (this.start >= other.start && this.start <= other.end) || (this.end >= other.start && this.end <= other.end);
        }

    }
}
