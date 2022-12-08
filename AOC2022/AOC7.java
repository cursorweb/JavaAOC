package AOC2022;

import Util.ReadFile;

import java.util.ArrayList;
import java.util.List;

public class AOC7 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 7);
        String[] lines = file.split("\\n");

        Folder rootFolder = new Folder("");
        ArrayList<String> path = new ArrayList<>();

        int i = 0;

        for (String line : lines) {
            if (!line.startsWith("$")) {
                i++;
                continue;
            }
            // "$ "
            String command = line.substring(2);
            if (command.startsWith("cd")) {
                // "cd "
                String dir = command.substring(3);
                if (dir.equals("/")) {
                    path.clear();
                } else if (dir.equals("..")) {
                    path.remove(path.size() - 1);
                } else {
                    path.add(dir);
                }
            } else { // must be ls
                ArrayList<String> outputs = new ArrayList<>();

                for (int j = i + 1; j < lines.length; j++) {
                    String output = lines[j];

                    if (output.startsWith("$")) break;
                    outputs.add(output);
                }

                for (String output : outputs) {
                    if (output.startsWith("dir")) {
                        // "dir "
                        String name = output.substring(4);
                        rootFolder.addItem(path, new Folder(name));
                    } else {
                        String[] split = output.split(" ");
                        int size = Integer.parseInt(split[0]);
                        String name = split[1];

                        rootFolder.addItem(path, new File(name, size));
                    }
                }
            }

            i++;
        }

        ArrayList<Integer> dirs = new ArrayList<>();
        System.out.println("Part1: " + totalSum(rootFolder, dirs));

        int totalSpace = 70_000_000;
        int minUnusedSpace = 30_000_000;

        int currentlyUsing = rootFolder.getSize();
        int currentlyUnused = totalSpace - currentlyUsing;
        int minSize = minUnusedSpace - currentlyUnused;

        int min = totalSpace;

        for (int dir : dirs) {
            if (dir < minSize) {
                continue;
            }

            if (dir < min) {
                min = dir;
            }
        }

        System.out.println("Part2: " + min);
    }

    private static int totalSum(Folder f, ArrayList<Integer> sizes) {
        for (Item i : f.items) {
            if (i instanceof Folder) {
                sizes.add(((Folder) i).getSize());
                totalSum((Folder) i, sizes);
            }
        }

        int sum = 0;
        for (int x : sizes) {
            if (x < 100_000) {
                sum += x;
            }
        }

        return sum;
    }
}

abstract class Item {
    public String name;

    public abstract String toString(int amt);
}

class File extends Item {
    public int size;

    public File(String name, int size) {
        this.name = name;
        this.size = size;
    }

    public String toString() {
        return toString(0);
    }

    public String toString(int amt) {
        return name + " (size=" + size + ")";
    }
}

class Folder extends Item {
    public ArrayList<Item> items;

    public Folder(String name) {
        this.name = name;
        this.items = new ArrayList<>();
    }

    public void addItem(List<String> path, Item item) {
        if (path.size() == 0) {
            items.add(item);
        } else {
            String path0 = path.get(0);

            for (Item i : items) {
                if (i.name.equals(path0)) {
                    ((Folder) i).addItem(path.subList(1, path.size()), item);
                    return;
                }
            }

            // not found
            Folder folder = new Folder(path0);
            items.add(folder);
            folder.addItem(path.subList(1, path.size()), item);
        }
    }

    public int getSize() {
        int sum = 0;

        for (Item item : items) {
            if (item instanceof Folder) {
                sum += ((Folder) item).getSize();
            } else {
                sum += ((File) item).size;
            }
        }

        return sum;
    }

    public String toString() {
        return toString(1);
    }

    public String toString(int amt) {
        StringBuilder out = new StringBuilder(name + "/");

        out.append("\n");
        for (Item itm : items) {
            out.append(new String(new char[amt]).replace("\0", "  ")).append(itm.toString(amt + 1));
            out.append("\n");
        }

        return out.toString();
    }
}