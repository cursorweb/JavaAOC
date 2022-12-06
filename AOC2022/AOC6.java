package AOC2022;

import Util.ReadFile;

import java.util.ArrayList;

public class AOC6 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 6);

        for (int i = 0; i < file.length() - 4; i++) {
            String word = file.substring(i, i + 4);
            if (isUnique(word)) {
                System.out.println("Part1: " + (i + 4));
                break;
            }
        }

        for (int i = 0; i < file.length() - 14; i++) {
            String word = file.substring(i, i + 14);
            if (isUnique(word)) {
                System.out.println("Part2: " + (i + 14));
                break;
            }
        }
    }

    private static boolean isUnique(String xs) {
        ArrayList<Character> found = new ArrayList<>();
        for (char x : xs.toCharArray()) {
            if (!found.contains(x)) {
                found.add(x);
                continue;
            }

            return false;
        }

        return true;
    }
}
