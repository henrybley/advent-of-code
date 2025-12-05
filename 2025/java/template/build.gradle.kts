plugins {
    application
}

java {
    toolchain.languageVersion.set(JavaLanguageVersion.of(21))
}

application {
    // Default runner: override this if you want to run Part2 instead
    mainClass.set("aoc.Part1")
}

repositories {
    mavenCentral()
}

