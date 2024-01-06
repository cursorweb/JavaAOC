package AOC2020.AOC17;

import util.ReadFile;

public class Main {
    public static void main(String[] args) {
        String file = ReadFile.from(2020, 17);
        String[] lines = file.split("\\n");

        int size = lines[0].length();

        {
            PointGrid3D grid3D = new PointGrid3D(size);

            int y = 0;

            for (String line : lines) {
                char[] chars = line.toCharArray();
                for (int x = 0; x < chars.length; x++) {
                    boolean on = chars[x] == '#';
                    grid3D.addPoint(new Point3D(x - size / 2, y - size / 2, 0), on);
                }
                y++;
            }

            for (int i = 0; i < 6; i++) {
                grid3D.update();
            }

            System.out.println("Part1: " + grid3D.findActive());
        }

        {
            PointGrid4D grid4D = new PointGrid4D(size);

            int y = 0;

            for (String line : lines) {
                char[] chars = line.toCharArray();
                for (int x = 0; x < chars.length; x++) {
                    boolean on = chars[x] == '#';
                    grid4D.addPoint(new Point4D(x - size / 2, y - size / 2, 0, 0), on);
                }
                y++;
            }

            for (int i = 0; i < 6; i++) {
                grid4D.update();
            }

            System.out.println("Part2: " + grid4D.findActive());
        }
    }
}
