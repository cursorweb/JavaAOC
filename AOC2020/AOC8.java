package AOC2020;

import util.ReadFile;

import java.util.ArrayList;

public class AOC8 {
    public static void main(String[] args) {
        String file = ReadFile.from(2020, 8);
        String[] lines = file.split("\\n");

        int acc = 0;
        int i = 0;

        ArrayList<Integer> instructions = new ArrayList<>();

        while (!instructions.contains(i)) {
            instructions.add(i);

            String line = lines[i];

            String[] lineCodes = line.split(" ");
            String instruction = lineCodes[0];
            int sign = lineCodes[1].charAt(0) == '+' ? 1 : -1;
            int num = Integer.parseInt(lineCodes[1].substring(1));

            if (instruction.equals("nop")) {
                i++;
            }

            if (instruction.equals("acc")) {
                acc += sign * num;
                i++;
            }

            if (instruction.equals("jmp")) {
                i += sign * num;
            }
        }

        System.out.println("Part1: " + acc);

        // part 2 is special

        // this is the first time we brute force
        // step 1: catalog the instructions, and mark the points of interest
        ArrayList<Instr> instrs = new ArrayList<>();
        ArrayList<Integer> pois = new ArrayList<>();

        for (int is = 0; is < lines.length; is++) {
            String line = lines[is];
            String[] lineCodes = line.split(" ");
            String instruction = lineCodes[0];
            int num = Integer.parseInt(lineCodes[1]);

            if (instruction.equals("nop")) {
                pois.add(is);
                instrs.add(new Instr(Type.Nop, num));
            }

            if (instruction.equals("acc")) {
                instrs.add(new Instr(Type.Acc, num));
            }

            if (instruction.equals("jmp")) {
                pois.add(is);
                instrs.add(new Instr(Type.Jmp, num));
            }
        }

        // step 2:
        // loop through each poi
        // flip the poi
        // execute it
        for (int poi : pois) {
            instrs.get(poi).flip();

            int result = exec(instrs);
            if (result != -1) {
                System.out.println("Part2: " + result);
                break;
            }

            instrs.get(poi).flip(); // don't forget to undo
        }
    }


    // -1 = fail, otherwise part2!
    private static int exec(ArrayList<Instr> instrs) {
        ArrayList<Integer> visited = new ArrayList<>();

        int acc = 0;
        int i = 0;

        while (!visited.contains(i) && i < instrs.size()) {
            visited.add(i);

            Instr instr;

            try {
                instr = instrs.get(i);
            } catch (Exception e) {
                return -1;
            }

            if (instr.type == Type.Jmp) {
                i += instr.num;
                continue;
            }

            if (instr.type == Type.Acc) {
                acc += instr.num;
            }

            i++;
        }

        // the loop will do one extra + 1
        if (i == instrs.size()) {
            return acc;
        }

        return -1;
    }
}

class Instr {
    public Type type;
    public int num;

    public Instr(Type t, int n) {
        type = t;
        num = n;
    }

    public void flip() {
        if (type == Type.Jmp) {
            type = Type.Nop;
        } else if (type == Type.Nop) {
            type = Type.Jmp;
        }
    }

    @Override
    public String toString() {
        return type + " " + num;
    }
}

enum Type {
    Jmp,
    Nop,
    Acc
}