package aoc;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

public class Util {

    public static List<String> read(String day) {
        String folder = "../../problems/" + day;
        Path path = Path.of(folder).toAbsolutePath();

        try {
            return Files.readAllLines(path);
        } catch (Exception e) {
            throw new RuntimeException("Failed to read input at: " + path, e);
        }
    }
}
