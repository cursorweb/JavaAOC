package AOC2020;

import Util.ReadFile;

import java.util.HashMap;

public class AOC6 {
    public static void main(String[] args) {
        String file = ReadFile.from(2020, 6);

        String[] lines = file.split("\\n\\n");

        int sum = 0;

        for (String line : lines) {
            HashMap<Character, Boolean> answers = new HashMap<>();

            // quick hack
            for (char c : line.replaceAll("\\W", "").toCharArray()) {
                if (!answers.containsKey(c)) {
                    answers.put(c, true);
                }
            }

            sum += answers.size();
        }

        System.out.println("Part1: " + sum);

        int sum2 = 0;
        for (String line : lines) {
            String[] peoples = line.split("\\n");
            int len = peoples.length;
            HashMap<Character, Integer> answers = new HashMap<>();

            for (char c : line.replaceAll("\\W", "").toCharArray()) {
                if (!answers.containsKey(c)) {
                    answers.put(c, 0);
                }

                answers.put(c, answers.get(c) + 1);
            }

            int commons = 0;
            for (int c : answers.values()) {
                if (c == len) {
                    commons++;
                }
            }

            sum2 += commons;
        }

        System.out.println("Part2: " + sum2);
    }
}
