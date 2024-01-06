package AOC2023;

import util.ReadFile;

import java.util.HashMap;
import java.util.Objects;

public class AOC3 {
    public static void main(String[] args) {
        String file = ReadFile.from(2023, 3);
        String[] lines = file.split("\n");

        HashMap<Point, Character> symbols = new HashMap<>();
        HashMap<Point, String> numbers = new HashMap<>();

        for (int y = 0; y < lines.length; y++) {
            String line = lines[y];
            StringBuilder num = new StringBuilder();
            int numx = -1;

            for (int x = 0; x < line.length(); x++) {
                char c = line.charAt(x);

                if ('0' <= c && c <= '9') {
                    if (numx == -1) {
                        numx = x;
                    }

                    num.append(c);
                } else {
                    if (numx != -1) {
                        numbers.put(new Point(y, numx), num.toString());

                        numx = -1;
                        num = new StringBuilder();
                    }

                    if (c != '.') {
                        symbols.put(new Point(y, x), c);
                    }
                }
            }

            if (numx != -1) {
                numbers.put(new Point(y, numx), num.toString());
            }
        }

        int sum = 0;
        search:
        for (Point pos : numbers.keySet()) {
            String number = numbers.get(pos);

            for (int y = pos.y - 1; y <= pos.y + 1; y++) {
                for (int x = pos.x - 1; x <= pos.x + number.length(); x++) {
                    if (symbols.containsKey(new Point(y, x))) {
                        sum += Integer.parseInt(number);
                        continue search;
                    }
                }
            }
        }

        System.out.println("Part1: " + sum);

        int gearRatios = 0;
        for (Point pos : symbols.keySet()) {
            if (symbols.get(pos) != '*') {
                continue;
            }

            int number1 = -1;
            int number2 = -1;

            for (Point numPos : numbers.keySet()) {
                String number = numbers.get(numPos);

                if (Math.abs(numPos.y - pos.y) <= 1 && (pos.x >= numPos.x - 1 && pos.x <= numPos.x + number.length())) {
                    if (number1 == -1) {
                        number1 = Integer.parseInt(number);
                    } else {
                        number2 = Integer.parseInt(number);
                    }
                }
            }

            if (number1 == -1 || number2 == -1) {
                continue;
            }

            gearRatios += number1 * number2;
        }

        System.out.println("Part2: " + gearRatios);
    }
}

class Point {
    public int y;
    public int x;

    Point(int y, int x) {
        this.y = y;
        this.x = x;
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
        return Objects.hash(y, x);
    }
}