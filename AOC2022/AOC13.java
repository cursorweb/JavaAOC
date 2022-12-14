package AOC2022;

import Util.ReadFile;

import java.util.ArrayList;

public class AOC13 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 13);
        String[] lines = file.split("\\n\\n");

        ArrayList<Array[]> arrObjs = new ArrayList<>();

        for (String line : lines) {
            String[] arrays = line.split("\n");
            Array left = Array.from(arrays[0].substring(1, arrays[0].length() - 1));
            Array right = Array.from(arrays[1].substring(1, arrays[1].length() - 1));

            arrObjs.add(new Array[]{left, right});
        }

        int sum = 0;
        int i = 0;
        for (Array[] arr : arrObjs) {
            Array left = arr[0];
            Array right = arr[1];

            if (left.compare(right) == -1) {
                sum += i + 1;
            }

            i++;
//            System.out.println("\n\n");
        }

        System.out.println("Part1: " + sum);
    }
}


abstract class Branch {
    // -1 = A < B
    // 0 = [] -> []??
    // 1 = A > B
    public abstract int compare(Branch b);
}

class Number extends Branch {
    int val;

    public Number(int val) {
        this.val = val;
    }

    @Override
    public int compare(Branch b) {
//        System.out.println("Compare " + val + " and " + b);
        if (b instanceof Number) {
            return (int) Math.signum(val - ((Number) b).val);
        } else {
            return new Array(val).compare(b);
        }
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

    public Array(int val) {
        this.val = new ArrayList<>();
        this.val.add(new Number(val));
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
    public int compare(Branch bObj) {
//        System.out.println("Comparing " + val + " " + bObj);
        if (bObj instanceof Array) {
            Array b = (Array) bObj;
            for (int i = 0; i < Math.min(val.size(), b.val.size()); i++) {
//                System.out.println("Compare " + val.get(i) + " and " + b.val.get(i));

                int result = val.get(i).compare(b.val.get(i));

                if (result == 1) {
//                    System.out.println("Wrong order");
                    return 1;
                }

                if (result == -1) {
//                    System.out.println("Right order");
                    return -1;
                }
            }

            if (val.size() != b.val.size()) {
//                System.out.println("Different array size");
//                System.out.println("So: " + (((int)Math.signum(val.size() - b.val.size()) == -1) ? "Right order" : "Wrong order"));
                return (int)Math.signum(val.size() - b.val.size());
            }

//            System.out.println("No order");
            return 0;
        } else {
            Number b = (Number) bObj;
//            System.out.println("Cast left to array: " + b.val);
            return compare(new Array(b.val));
        }
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