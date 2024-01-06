package util;

import java.io.InputStream;
import java.util.Objects;
import java.util.Scanner;

public class ReadFile {
    public static String from(int year, int num) {
        InputStream is = Objects.requireNonNull(ReadFile.class.getClassLoader().getResourceAsStream("AOC" + year + "/AOC" + num + ".txt"));
        Scanner scan = new Scanner(is);

        StringBuilder out = new StringBuilder();
        while (scan.hasNextLine()) {
            out.append(scan.nextLine());
            out.append("\n");
        }

        scan.close();

        return out.toString();
    }
}
