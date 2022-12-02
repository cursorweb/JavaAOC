package AOC2020.AOC17;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.Objects;

class Point4D {
    public int x;
    public int y;
    public int z;
    public int w;

    public Point4D(int x, int y, int z, int w) {
        this.x = x;
        this.y = y;
        this.z = z;
        this.w = w;
    }

    /*
    During a cycle, all cubes simultaneously change their state according to the following rules:
    If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
    If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
    */
    public boolean changeState(HashMap<Point4D, Boolean> points) {
        boolean active = points.get(this);
        int actives = 0;

        for (int sx = x - 1; sx <= x + 1; sx++) {
            for (int sy = y - 1; sy <= y + 1; sy++) {
                for (int sz = z - 1; sz <= z + 1; sz++) {
                    for (int sw = w - 1; sw <= w + 1; sw++) {
                        if (sx == x && sy == y && sz == z && sw == w) {
                            continue;
                        }

                        Point4D p = new Point4D(sx, sy, sz, sw);
                        if (points.containsKey(p) && points.get(p)) {
                            actives++;
                        }
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
        Point4D point = (Point4D) o;
        return x == point.x && y == point.y && z == point.z && w == point.w;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y, z, w);
    }

    @Override
    public String toString() {
        return "(" + x + ", " + y + ", " + z + ", " + w + ")";
    }
}

class PointGrid4D {
    public HashMap<Point4D, Boolean> points = new HashMap<>();
    public int size;
    public int zwSize = 1;

    public PointGrid4D(int size) {
        this.size = size;
    }

    public void addPoint(Point4D p, boolean on) {
        points.put(p, on);
    }

    public void update() {
        // expand by 1 dimension
        size += 2;
        zwSize += 2;
        for (int x = -size / 2; x <= size / 2; x++) {
            for (int y = -size / 2; y <= size / 2; y++) {
                for (int z = -zwSize / 2; z <= zwSize / 2; z++) {
                    for (int w = -zwSize / 2; w <= zwSize / 2; w++) {
                        Point4D p = new Point4D(x, y, z, w);
                        if (!points.containsKey(p)) {
                            points.put(p, false);
                        }
                    }
                }
            }
        }

        HashMap<Point4D, Boolean> newPoints = new HashMap<>();
        for (Point4D point : points.keySet()) {
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

    private String formatMap(HashMap<Point4D, Boolean> points) {
        // z : [spread]
        HashMap<Integer, ArrayList<Point4D>> parsed = new HashMap<>();

        for (Point4D p : points.keySet()) {
            int z = p.z;

            if (!parsed.containsKey(z)) {
                parsed.put(z, new ArrayList<>());
            }

            ArrayList<Point4D> grid = parsed.get(z);
            grid.add(p);
        }

        StringBuilder out = new StringBuilder();

        for (int z : parsed.keySet().stream().sorted(Integer::compareTo).toList()) {
            // w, [scatter of x and y]
            HashMap<Integer, ArrayList<Point4D>> parsed4D = new HashMap<>();

            for (Point4D p : points.keySet()) {
                int w = p.w;

                if (!parsed4D.containsKey(w)) {
                    parsed4D.put(w, new ArrayList<>());
                }

                ArrayList<Point4D> grid = parsed4D.get(w);
                grid.add(p);
            }

            for (int w : parsed.keySet().stream().sorted(Integer::compareTo).toList()) {
                char[][] grid = new char[size * 2 + 1][size * 2 + 1];

                ArrayList<Point4D> pointList = parsed4D.get(w);
                for (Point4D p : pointList) {
                    grid[p.y + size / 2][p.x + size / 2] = points.get(p) ? '#' : '.';
                }

                StringBuilder gridS = new StringBuilder();

                for (int i = 0; i < size; i++) {
                    for (int j = 0; j < size; j++) {
                        gridS.append(grid[i][j]);
                    }
                    gridS.append("\n");
                }

                out.append("z = ").append(z).append(", w = ").append(w).append("\n");
                out.append(gridS);
                out.append("\n\n");
            }
        }

        return out.toString();
    }
}