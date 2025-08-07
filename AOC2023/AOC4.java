package AOC2023;

import util.ReadFile;

import java.util.ArrayList;
import java.util.Arrays;

public class AOC4 {
    public static void main(String[] args) {
        String file = ReadFile.from(2023, 4);
        String[] lines = file.split("\n");

        int[] cardCounts = new int[lines.length];
        Arrays.fill(cardCounts, 1);

        int totalPoints = 0;

        for (int cardIdx = 0; cardIdx < lines.length; cardIdx++) {
            String line = lines[cardIdx];
            String[] split = line.split(" +");

            ArrayList<Integer> winningNumbers = new ArrayList<>();
            ArrayList<Integer> yourNumbers = new ArrayList<>();

            boolean onWinning = true;

            // "Card" "1: "
            for (int i = 2; i < split.length; i++) {
                String val = split[i];
                if (val.equals("|")) {
                    onWinning = false;
                    continue;
                }

                if (onWinning) {
                    winningNumbers.add(Integer.parseInt(val));
                } else {
                    yourNumbers.add(Integer.parseInt(val));
                }
            }

            int pow = -1;

            for (int num : yourNumbers) {
                if (winningNumbers.contains(num)) {
                    pow++;
                }
            }

            if (pow > -1) {
                totalPoints += Math.pow(2, pow);
            }

            int matchingNumbers = pow + 1;
            int cardCount = cardCounts[cardIdx];
            for (int i = cardIdx + 1; i < cardIdx + 1 + matchingNumbers; i++) {
                cardCounts[i] += cardCount;
            }
        }

        System.out.println("Part1: " + totalPoints);

        int sum = 0;
        for (int cardCount : cardCounts) {
            sum += cardCount;
        }
        System.out.println("Part2: " + sum);
    }
}
