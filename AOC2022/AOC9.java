package AOC2022;

import util.ReadFile;

import java.util.HashSet;

public class AOC9 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 9);
        String[] lines = file.split("\\n");


        // part1
        HashSet<Point> visitedSingle = new HashSet<>();
        Point tailSingle = new Point();
        Point headSingle = new Point();


        // part 2
        HashSet<Point> visitedMulti = new HashSet<>();
        Point[] ropeMulti = new Point[10];

        for (int i = 0; i < 10; i++) {
            ropeMulti[i] = new Point();
        }

        Point tailMulti = ropeMulti[0];
        Point headMulti = ropeMulti[9];


        // add initial states
        visitedSingle.add(tailSingle);
        visitedMulti.add(tailMulti);

        for (String line : lines) {
            String[] split = line.split(" ");
            char dir = line.charAt(0);
            int amt = Integer.parseInt(split[1]);

            for (int i = 0; i < amt; i++) {
                // part1
                headSingle.move(dir);
                if (tailSingle.notTouches(headSingle)) {
                    tailSingle.catchup(headSingle);
                    visitedSingle.add(tailSingle);
                }

                // part2
                headMulti.move(dir);
                for (int j = ropeMulti.length - 2; j >= 0; j--) {
                    Point p = ropeMulti[j];
                    Point f = ropeMulti[j + 1];
                    if (p.notTouches(f)) {
                        p.catchup(f);

                        if (j == 0) {
                            visitedMulti.add(tailMulti);
                        }
                    }
                }
            }
        }

        System.out.println("Part1: " + visitedSingle.size());
        System.out.println("Part2: " + visitedMulti.size());
    }

    private static class Point {
        public int x;
        public int y;

        public Point() {
            x = 0;
            y = 0;
        }

        public boolean notTouches(Point other) {
            return Math.abs(other.x - x) > 1 || Math.abs(other.y - y) > 1;
        }

        public void move(char dir) {
            switch (dir) {
                case 'U':
                    y++;
                    break;
                case 'D':
                    y--;
                    break;
                case 'L':
                    x--;
                    break;
                case 'R':
                    x++;
                    break;
            }
        }

        public void catchup(Point other) {
            if (x < other.x) {
                x++;
            }

            if (x > other.x) {
                x--;
            }

            if (y < other.y) {
                y++;
            }

            if (y > other.y) {
                y--;
            }
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            Point point = (Point) o;
            return x == point.x && y == point.y;
        }

        @Override
        public int hashCode() {
            return toString().hashCode();
        }

        @Override
        public String toString() {
            return x + ", " + y;
        }
    }
}
