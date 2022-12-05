package AOC2022;

import Util.ReadFile;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;

public class AOC5 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 5);
        String[] lines = file.split("\\n");

        ArrayList<String> crateLines = new ArrayList<>();
        ArrayList<String> instrs = new ArrayList<>();
        HashMap<Integer, ArrayList<Character>> buckets1 = new HashMap<>();
        HashMap<Integer, ArrayList<Character>> buckets2 = new HashMap<>();

        // parse lines
        {
            boolean isInstrs = false;
            int i = 0;
            while (i < lines.length) {
                String line = lines[i];
                if (line.equals("")) {
                    isInstrs = true;
                    i++;
                    continue;
                }

                if (!isInstrs) {
                    crateLines.add(line);
                } else {
                    instrs.add(line);
                }

                i++;
            }
        }

        Collections.reverse(crateLines);

        // parse crates
        for (String line : crateLines.subList(1, crateLines.size())) {
            int ci = 0;
            for (int i = 1; i < line.length(); i += 4) {
                char x = line.charAt(i);

                if (!buckets1.containsKey(ci)) {
                    buckets1.put(ci, new ArrayList<>());
                    buckets2.put(ci, new ArrayList<>());
                }

                if (x != ' ') {
                    ArrayList<Character> newArr = buckets1.get(ci);
                    newArr.add(x);

                    ArrayList<Character> newArr1 = buckets2.get(ci);
                    newArr1.add(x);
                }

                ci++;
            }
        }

        for (String instr : instrs) {
            int movel = "move ".length();
            int movei = instr.indexOf("move ");

            int froml = " from ".length();
            int fromi = instr.indexOf(" from ");

            int tol = " to ".length();
            int toi = instr.indexOf(" to ");

            int move = Integer.parseInt(instr.substring(movei + movel, fromi));
            int from = Integer.parseInt(instr.substring(fromi + froml, toi));
            int to = Integer.parseInt(instr.substring(toi + tol));

            ArrayList<Character> crates = new ArrayList<>();
            for (int i = 0; i < move; i++) {
                int idx = buckets1.get(from - 1).size() - 1;
                crates.add(buckets1.get(from - 1).get(idx));
                buckets1.get(from - 1).remove(idx);
            }

            buckets1.get(to - 1).addAll(crates);
        }

        StringBuilder part1 = new StringBuilder();

        for (int i = 0; i < buckets1.size(); i++) {
            ArrayList<Character> bucket = buckets1.get(i);
            part1.append(bucket.get(bucket.size() - 1));
        }

        System.out.println("Part1: " + part1);
    }
}
