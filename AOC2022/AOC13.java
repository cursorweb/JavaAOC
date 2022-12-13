package AOC2022;

import Util.ReadFile;

import java.util.ArrayList;

public class AOC13 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 13);
        String[] lines = file.split("\\n\\n");

        for (String line : lines) {
            String[] arrays = line.split("\n");
            Array left = Array.from(arrays[0].substring(1, arrays[0].length() - 1));
            Array right = Array.from(arrays[1].substring(1, arrays[1].length() - 1));

            System.out.println(left + "\n" + right + "\n");
        }
    }
}


abstract class Branch { }

class Number extends Branch {
    int val;

    public Number(int val) {
        this.val = val;
    }

    @Override
    public String toString() {
        return val + "";
    }
}

class Array extends Branch {
    ArrayList<Branch> val;

    public Array(ArrayList<Branch> val) {
        this.val = val;
    }

    public static Array from(String x) {
        if (x.length() == 0) {
            return new Array(new ArrayList<>());
        }

        ArrayList<Branch> branches = new ArrayList<>();
        ArrayList<String> items = new ArrayList<>();
        int nest = 0;
        StringBuilder current = new StringBuilder();

        for (char c : x.toCharArray()) {
            if (c == '[') {
                nest++;
            } else if (c == ']') {
                nest--;
            } else if (c == ',' && nest == 0) {
                items.add(current.toString());
                current = new StringBuilder();
                continue;
            }

            current.append(c);
        }

        // add the last one w/o comma
        items.add(current.toString());

        for (String item : items) {
            if (!item.startsWith("[")) {
                branches.add(new Number(Integer.parseInt(item)));
            } else {
                branches.add(from(item.substring(1, item.length() - 1)));
            }
        }

        return new Array(branches);
    }

    @Override
    public String toString() {
        StringBuilder out = new StringBuilder("{");

        for (Branch x : val) {
            out.append(x);
            out.append(",");
        }

        out.append("}");

        return out.toString();
    }
}