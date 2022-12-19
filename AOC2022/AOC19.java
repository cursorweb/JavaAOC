package AOC2022;

import Util.ReadFile;

public class AOC19 {
    public static void main(String[] args) {
        String file = ReadFile.from(2022, 19);
        String[] lines = file.split("\\n");

        for (String blueprintStr : lines) {
            String robotsStr = blueprintStr.split(": ")[1];
            System.out.println(robotsStr);
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
