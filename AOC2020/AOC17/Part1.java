package AOC2020.AOC17;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.Objects;

class Point3D {
    public int x;
    public int y;
    public int z;

    public Point3D(int x, int y, int z) {
        this.x = x;
        this.y = y;
        this.z = z;
    }

    /*
    During a cycle, all cubes simultaneously change their state according to the following rules:
    If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
    If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
    */
    public boolean changeState(HashMap<Point3D, Boolean> points) {
        boolean active = points.get(this);
        int actives = 0;

        for (int sx = x - 1; sx <= x + 1; sx++) {
            for (int sy = y - 1; sy <= y + 1; sy++) {
                for (int sz = z - 1; sz <= z + 1; sz++) {
                    if (sx == x && sy == y && sz == z) {
                        continue;
                    }

                    Point3D p = new Point3D(sx, sy, sz);
                    if (points.containsKey(p) && points.get(p)) {
                        actives++;
                    }
                }
            }
        }

        if (active) {
            return actives == 2 || actives == 3;
        } else {
            return actives == 3;
        }
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Point3D point = (Point3D) o;
        return x == point.x && y == point.y && z == point.z;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y, z);
    }

    @Override
    public String toString() {
        return "(" + x + ", " + y + ", " + z + ")";
    }
}

class PointGrid3D {
    public HashMap<Point3D, Boolean> points = new HashMap<>();
    public int size;
    public int zSize = 1;

    public PointGrid3D(int size) {
        this.size = size;
    }

    public void addPoint(Point3D p, boolean on) {
        points.put(p, on);
    }

    public void update() {
        // expand by 1 dimension
        size += 2;
        zSize += 2;
        for (int x = -size / 2; x <= size / 2; x++) {
            for (int y = -size / 2; y <= size / 2; y++) {
                for (int z = -zSize / 2; z <= zSize / 2; z++) {
                    Point3D p = new Point3D(x, y, z);
                    if (!points.containsKey(p)) {
                        points.put(p, false);
                    }
                }
            }
        }

        HashMap<Point3D, Boolean> newPoints = new HashMap<>();
        for (Point3D point : points.keySet()) {
            boolean newState = point.changeState(points);
            newPoints.put(point, newState);
        }

        points = newPoints;
    }

    public int findActive() {
        int out = 0;

        for (boolean b : points.values()) {
            if (b) out++;
        }

        return out;
    }

    @Override
    public String toString() {
        return formatMap(points);
    }

    private String formatMap(HashMap<Point3D, Boolean> points) {
        // z : [spread]
        HashMap<Integer, ArrayList<Point3D>> parsed = new HashMap<>();

        for (Point3D p : points.keySet()) {
            int z = p.z;

            if (!parsed.containsKey(z)) {
                parsed.put(z, new ArrayList<>());
            }

            ArrayList<Point3D> grid = parsed.get(z);
            grid.add(p);
        }

        StringBuilder out = new StringBuilder();

        for (int z : parsed.keySet().stream().sorted(Integer::compareTo).toList()) {
            char[][] grid = new char[size * 2 + 1][size * 2 + 1];

            ArrayList<Point3D> pointList = parsed.get(z);
            for (Point3D p : pointList) {
                grid[p.y + size / 2][p.x + size / 2] = points.get(p) ? '#' : '.';
            }

            StringBuilder gridS = new StringBuilder();

            for (int i = 0; i < size; i++) {
                for (int j = 0; j < size; j++) {
                    gridS.append(grid[i][j]);
                }
                gridS.append("\n");
            }

            out.append("z = ").append(z).append("\n");
            out.append(gridS);
            out.append("\n");
        }

        return out.toString();
    }
}