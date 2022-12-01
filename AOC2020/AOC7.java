package AOC2020;

import Util.ReadFile;

import java.util.ArrayList;
import java.util.HashMap;

public class AOC7 {
    public static void main(String[] args) {
        String file = ReadFile.from(2020, 7);
        String[] lines = file.split("\\n");

        HashMap<String, ArrayList<String>> bagDict = new HashMap<>();

        for (String line : lines) {
            String containSep = " bags contain ";
            int containIndex = line.indexOf(containSep);
            String bag = line.substring(0, containIndex);

            String rest = line.substring(containIndex + containSep.length(), line.length() - 1); // remove '.'

            if (rest.equals("no other bags")) {
                bagDict.put(bag, new ArrayList<>());
                continue;
            }

            String[] restBags = rest.split(", ");

            ArrayList<String> restBagArr = new ArrayList<>();
            for (String restBag : restBags) {
                int firstSpace = restBag.indexOf(' ');
                int firstBag = restBag.indexOf("bag");

                // exclude the space(s)
                restBagArr.add(restBag.substring(firstSpace + 1, firstBag - 1));
            }

            bagDict.put(bag, restBagArr);
        }

        int count = 0;

        for (String key : bagDict.keySet()) {
            if (key.equals("shiny gold")) {
                continue;
            }

            ArrayList<String> bags = bagDict.get(key);

            if (bags.contains("shiny gold")) {
                count++;
                continue;
            }

            for (String bag : bags) {
                if (doesContain(bagDict, bag)) {
                    count++;
                    break;
                }
            }
        }

        System.out.println("Part1: " + count);

        HashMap<String, ArrayList<Pair>> numDict = new HashMap<>();

        for (String line : lines) {
            String containSep = " bags contain ";
            int containIndex = line.indexOf(containSep);
            String bag = line.substring(0, containIndex);

            String rest = line.substring(containIndex + containSep.length(), line.length() - 1); // remove '.'

            if (rest.equals("no other bags")) {
                numDict.put(bag, new ArrayList<>());
                continue;
            }

            String[] restBags = rest.split(", ");

            ArrayList<Pair> restBagArr = new ArrayList<>();
            for (String restBag : restBags) {
                int firstSpace = restBag.indexOf(' ');
                int firstBag = restBag.indexOf("bag");

                int num = Integer.parseInt(restBag.substring(0, firstSpace));

                // exclude the space(s)
                restBagArr.add(new Pair(restBag.substring(firstSpace + 1, firstBag - 1), num));
            }

            numDict.put(bag, restBagArr);
        }

        int bagsCont = bagsInside(numDict, "shiny gold");
        System.out.println("Part2: " + bagsCont);
    }

    private static boolean doesContain(HashMap<String, ArrayList<String>> bagDict, String key) {
        ArrayList<String> bags = bagDict.get(key);

        if (bags.contains("shiny gold")) {
            return true;
        }

        for (String bag : bags) {
            if (doesContain(bagDict, bag)) {
                return true;
            }
        }

        return false;
    }

    private static int bagsInside(HashMap<String, ArrayList<Pair>> bagsDict, String key) {
        ArrayList<Pair> bags = bagsDict.get(key);

        if (bags.size() == 0) {
            return 1;
        }

        // don't count yourself
        int result = key.equals("shiny gold") ? 0 : 1;

        for (Pair bag : bags) {
            result += bag.num * bagsInside(bagsDict, bag.bag);
        }

        return result;
    }
}

class Pair {
    public String bag;
    public int num;

    public Pair(String bag, int num) {
        this.bag = bag;
        this.num = num;
    }

    @Override
    public String toString() {
        return num + " " + bag;
    }
}