package aoc;

public class AdventOfCode {

    public static void main(String[] args) {

        boolean runPart1 = true;
        boolean runPart2 = true;

        if (args.length > 0) {
            switch (args[0]) {
                case "part1" -> {
                    runPart1 = true;
                    runPart2 = false;
                }
                case "part2" -> {
                    runPart1 = false;
                    runPart2 = true;
                }
                case "both" -> {
                    runPart1 = true;
                    runPart2 = true;
                }
                default -> {
                    System.out.println("Unknown argument: " + args[0]);
                    System.out.println("Use --part1, --part2, or --both");
                    return;
                }
            }
        }

        var input = Input.get();
        if (runPart1) {
            var result = Part1.solve(input);
            System.out.println("Part 1: " + result);
        }

        if (runPart2) {
            var input = Input.get();
            var result = Part2.solve(input);
            System.out.println("Part 2: " + result);
        }
    }
}

