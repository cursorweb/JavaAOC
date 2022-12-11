package AOC2022;

import Util.ReadFile;

public class AOC10 {
    private static int cycles = 0;
    private static int sum = 0;
    private static int X = 1;
    private static final StringBuilder grid = new StringBuilder();

    public static void main(String[] args) {
        String file = ReadFile.from(2022, 10);
        String[] lines = file.split("\\n");

        for (String line : lines) {
            String[] split = line.split(" ");
            String cmd = split[0];

            if (cmd.equals("noop")) {
                startCycle();
            } else {
                startCycle();
                int num = Integer.parseInt(split[1]);
                startCycle();
                X += num;
            }
        }
        
        System.out.println("Part1: " + sum);
        System.out.println("Part2:\n" + grid);
    }

    private static void startCycle() {
        cycles++;

        int c = cycles - 20;
        if (c % 40 == 0) {
            sum += X * cycles;
        }

        int pos = (cycles - 1) % 40;
        if (pos == 0) {
            grid.append("\n");
        }

        if (Math.abs(X - pos) <= 1) {
            grid.append("#");
        } else {
            grid.append(".");
        }
    }
}
