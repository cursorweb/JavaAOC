package AOC2022;

import util.ReadFile;

public class AOC19 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 19);
        String[] lines = file.split("\\n");

        int id = 0;
        for (String blueprintStr : lines) {
            String robotsStr = blueprintStr.split(": ")[1];
            System.out.println(robotsStr);
            id++;
        }
    }

    private static abstract class Robot {
        public int count;

        public Robot(int count) {
            this.count = count;
        }
    }

    private static class Blueprint {
        public int id;
        public OreRobot oreRobot;
        public ClayRobot clayRobot;
        public ObbyRobot obbyRobot;
        public GeodeRobot geodeRobot;

        public Blueprint(int id, OreRobot oreRobot, ClayRobot clayRobot, ObbyRobot obbyRobot, GeodeRobot geodeRobot) {
            this.id = id;
            this.oreRobot = oreRobot;
            this.clayRobot = clayRobot;
            this.obbyRobot = obbyRobot;
            this.geodeRobot = geodeRobot;
        }

        public int quality() {
            // loop 24 times = 24 minutes
            return id;
        }
    }

    private static class OreRobot {

    }

    private static class ClayRobot {

    }

    private static class ObbyRobot {

    }

    private static class GeodeRobot {

    }
}
