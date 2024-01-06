package AOC2022;

import util.ReadFile;

import java.util.HashMap;

public class AOC12 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 12);
        String[] lines = file.split("\\n");

        int height = lines.length;
        int width = lines[0].length();

        HashMap<Point, Character> points = new HashMap<>();

        for (int i = 0; i < height; i++) {
            for (int j = 0; j < width; j++) {
                points.put(new Point(j, i), lines[i].charAt(j));
            }
        }

        System.out.println(points);
    }

    private static
    class Point {
        int y;
        int x;

        public Point(int x, int y) {
            this.x = x;
            this.y = y;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            Point point = (Point) o;
            return y == point.y && x == point.x;
        }

        @Override
        public int hashCode() {
            return (y + "," + x).hashCode();
        }

        @Override
        public String toString() {
            return "(" + x + ", " + y + ")";
        }
    }
}
