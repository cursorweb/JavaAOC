package AOC2022;

import util.ReadFile;

import java.math.BigInteger;
import java.util.ArrayList;

public class AOC11 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 11);
        String[] lines = file.split("\\n\\n");

        ArrayList<Monkey> monkeys1 = new ArrayList<>();
        ArrayList<Monkey> monkeys2 = new ArrayList<>();

        ArrayList<Integer> inspection1 = new ArrayList<>();
        ArrayList<Integer> inspection2 = new ArrayList<>();

        int product = 1;

        for (String line : lines) {
            String[] codes = line.split("\n");

            String[] startingItems = codes[1].split(": ")[1].split(", ");

            ArrayList<Long> items = new ArrayList<>();
            ArrayList<Long> items2 = new ArrayList<>();
            for (String item : startingItems) {
                items.add(Long.parseLong(item));
                items2.add(Long.parseLong(item));
            }

            String op = codes[2].split(" = ")[1];
            int test = Integer.parseInt(codes[3].split(" by ")[1]);
            int ifTrue = Integer.parseInt(codes[4].split(" monkey ")[1]);
            int ifFalse = Integer.parseInt(codes[5].split(" monkey ")[1]);

            product *= test;

            monkeys1.add(new Monkey(items, op, test, ifTrue, ifFalse));
            monkeys2.add(new Monkey(items2, op, test, ifTrue, ifFalse));

            inspection1.add(0);
            inspection2.add(0);
        }

        for (int round = 0; round < 20; round++) {
            int i = 0;
            for (Monkey monkey : monkeys1) {
                ArrayList<Long> items = monkey.items;
                String operation = monkey.op;
                int test = monkey.test;

                char op = operation.charAt(4);

                for (long item : items) {
                    inspection1.set(i, inspection1.get(i) + 1);

                    long val;
                    if (operation.charAt(6) == 'o') {
                        val = item;
                    } else {
                        val = Integer.parseInt(operation.substring(6));
                    }

                    long newItem = item;

                    switch (op) {
                        case '+':
                            newItem += val;
                            break;
                        case '-':
                            newItem -= val;
                            break;
                        case '*':
                            newItem *= val;
                            break;
                        case '/':
                            newItem /= val;
                            break;
                    }

                    newItem /= 3;

                    if (newItem % test == 0) {
                        monkeys1.get(monkey.ifTrue).items.add(newItem);
                    } else {
                        monkeys1.get(monkey.ifFalse).items.add(newItem);
                    }
                }

                i++;

                monkey.items.clear();
            }
        }

        for (int round = 0; round < 10_000; round++) {
            int i = 0;
            for (Monkey monkey : monkeys2) {
                ArrayList<Long> items = monkey.items;
                String operation = monkey.op;
                int test = monkey.test;

                char op = operation.charAt(4);

                for (long item : items) {
                    inspection2.set(i, inspection2.get(i) + 1);

                    long val;
                    if (operation.charAt(6) == 'o') {
                        val = item;
                    } else {
                        val = Long.parseLong(operation.substring(6));
                    }

                    long newItem = item;

                    switch (op) {
                        case '+':
                            newItem += val;
                            break;
                        case '-':
                            newItem -= val;
                            break;
                        case '*':
                            newItem *= val;
                            break;
                        case '/':
                            newItem /= val;
                            break;
                    }

                    newItem %= product;

                    if (newItem % test == 0) {
                        monkeys2.get(monkey.ifTrue).items.add(newItem % product);
                    } else {
                        monkeys2.get(monkey.ifFalse).items.add(newItem % product);
                    }
                }

                i++;

                monkey.items.clear();
            }
        }

        inspection1.sort((a, b) -> -a.compareTo(b));
        System.out.println("Part1: " + inspection1.get(0) * inspection1.get(1));

        inspection2.sort((a, b) -> -a.compareTo(b));
        System.out.println("Part2: " + new BigInteger(String.valueOf(inspection2.get(0))).multiply(new BigInteger(String.valueOf(inspection2.get(1)))));

        // CRT: Chinese Remainder Theorem
    }

    private static
    class Monkey {
        public ArrayList<Long> items;

        public String op;

        public int test;

        public int ifTrue;
        public int ifFalse;

        public Monkey(ArrayList<Long> items, String op, int test, int ifTrue, int ifFalse) {
            this.items = items;
            this.op = op;
            this.test = test;
            this.ifTrue = ifTrue;
            this.ifFalse = ifFalse;
        }

        @Override
        public String toString() {
            return "Items: " + items + "\nOp: " + op + "\nTest: " + test + "\n  ifTrue: " + ifTrue + "\n  ifFalse: " + ifFalse;
        }
    }
}
