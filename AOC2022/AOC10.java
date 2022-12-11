package AOC2022;

import Util.ReadFile;

public class AOC10 {
    private static int cycles = 0;
    private static int sum = 0;
    private static int X = 1;

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
    }

    private static void startCycle() {
        cycles++;

        int c = cycles - 20;
        if (c % 40 == 0) {
            System.out.println(X + ", " + cycles);
            sum += X * cycles;
        }
    }
}
