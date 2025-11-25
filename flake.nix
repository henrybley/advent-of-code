{
  description = "Advent of Code - Rust and Java";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        # Script to create a new day
        mkDay = pkgs.writeShellScriptBin "aoc-new-day" ''
          set -e

          LANG=$1
          DAY=$2
          YEAR=''${3:-2025}
          SESSION=$AOC_SESSION

          if [ -z "$LANG" ] || [ -z "$DAY" ]; then
            echo "Usage: aoc-new-day <rust|java> <day> [year]"
            echo "Example: aoc-new-day rust 1"
            exit 1
          fi

          # Check if template exists (year-specific)
          TEMPLATE_DIR="$YEAR/$LANG/template"

          if [ ! -d "$TEMPLATE_DIR" ]; then
            echo "Error: Template directory not found: $TEMPLATE_DIR"
            echo "Expected structure:"
            echo "  $YEAR/"
            echo "    rust/"
            echo "      template/"
            echo "    java/"
            echo "      template/"
            echo ""
            echo "Create the template directory for $YEAR first!"
            exit 1
          fi

          if [ -z "$SESSION" ]; then
            echo "Error: AOC_SESSION environment variable not set"
            echo "Get your session cookie from adventofcode.com and set it:"
            echo "export AOC_SESSION='your-session-cookie'"
            exit 1
          fi

          # Format day with leading zero
          DAY_PADDED=$(printf "%02d" $DAY)
          DIR="$YEAR/$LANG/day-$DAY_PADDED"

          if [ -d "$DIR" ]; then
            echo "Directory $DIR already exists!"
            exit 1
          fi

          echo "Creating $DIR from template..."
          mkdir -p "$YEAR/$LANG"

          # Copy template
          cp -r "$TEMPLATE_DIR" "$DIR"

          # Replace placeholders in all files
          find "$DIR" -type f -exec sed -i \
            -e "s/{{DAY_PADDED}}/$DAY_PADDED/g" \
            {} \;

          # Download problem description and convert to markdown
          PROBLEMS_DIR="$YEAR/problems/day-$DAY_PADDED"
          PROBLEM_FILE="$PROBLEMS_DIR/description.md"

          if [ ! -f "$PROBLEM_FILE" ]; then
            echo "Downloading problem description..."
            mkdir -p "$PROBLEMS_DIR"
            
            ${pkgs.curl}/bin/curl -s \
              "https://adventofcode.com/$YEAR/day/$DAY" \
              -o "$PROBLEMS_DIR/day-$DAY_PADDED.html"
            
            if [ -s "$PROBLEMS_DIR/day-$DAY_PADDED.html" ]; then
              echo "Converting to markdown..."
              ${pkgs.pandoc}/bin/pandoc \
                -f html \
                -t markdown \
                --wrap=none \
                "$PROBLEMS_DIR/day-$DAY_PADDED.html" \
                -o "$PROBLEM_FILE"
              
              # Clean up HTML file
              rm "$PROBLEMS_DIR/day-$DAY_PADDED.html"
              echo "Problem description saved to $PROBLEM_FILE"
            else
              echo "Warning: Failed to download problem description"
              rm -f "$PROBLEMS_DIR/day-$DAY_PADDED.html"
            fi
          else
            echo "Problem description already exists at $PROBLEM_FILE"
          fi

          # Download input
          echo "Downloading input for day $DAY..."
          ${pkgs.curl}/bin/curl -s \
            -H "Cookie: session=$SESSION" \
            "https://adventofcode.com/$YEAR/day/$DAY/input" \
            -o "$PROBLEMS_DIR/input.txt"

          if [ ! -s "$PROBLEMS_DIR/input.txt" ]; then
            echo "Warning: Failed to download input or input is empty"
            rm "$PROBLEMS_DIR/input.txt"
            echo "Make sure your AOC_SESSION cookie is valid"
          else
            echo "Input downloaded successfully"
          fi



          # Language-specific post-processing
          case $LANG in
            rust)
              if [ -f "$DIR/Cargo.toml" ]; then
                cd "$DIR"
                ${pkgs.cargo}/bin/cargo generate-lockfile 2>/dev/null || true
                cd - > /dev/null
              fi
              echo "Created Rust solution in $DIR"
              echo ""
              echo "Run with:       aoc-run rust $DAY"
              echo "               or: cd $DIR && cargo run"
              echo "Benchmark with: aoc-bench rust $DAY"
              ;;
              
            java)
              echo "Created Java solution in $DIR"
              echo ""
              echo "Run with:       aoc-run java $DAY"
              echo "               or: cd $DIR && javac Solution.java && java Solution"
              echo "Benchmark with: aoc-bench java $DAY"
              ;;
              
            *)
              echo "Created solution in $DIR"
              ;;
          esac

          echo ""
          echo "Done! Start coding in $DIR"
        '';

        # Script to run a solution
        runTest = pkgs.writeShellScriptBin "aoc-test" ''
          LANG=$1
          DAY=$2
          PART=$3
          YEAR=''${4:-2025}
          echo "PART: $PART"

          if [ -z "$LANG" ] || [ -z "$DAY" ] || [ -z "$PART" ]; then
            echo "Usage: aoc-test <rust|java> <day> <part> [year]"
            echo "Examples:"
            echo "aoc-test rust 1 part2"
            echo "aoc-test rust 1 part1 2023"
            exit 1
          fi

          DAY_PADDED=$(printf "%02d" $DAY)
          DIR="$YEAR/$LANG/day-$DAY_PADDED"

          if [ ! -d "$DIR" ]; then
            echo "Error: Directory $DIR does not exist"
            exit 1
          fi

          cd "$DIR"

          case $LANG in
            rust)
              echo "Running Rust test for day $DAY..."
              ${pkgs.cargo}/bin/cargo test test_$PART
              ;;
            java)
              echo "Running Java test for day $DAY..."
              # Compile if needed
              if [ ! -f "Solution.class" ] || [ "Solution.java" -nt "Solution.class" ]; then
                ${pkgs.jdk}/bin/javac Solution.java
              fi
              ${pkgs.jdk}/bin/java Solution
              ;;
            *)
              echo "Unknown language: $LANG"
              exit 1
              ;;
          esac
        '';
        # Script to run a solution
        runSolution = pkgs.writeShellScriptBin "aoc-run" ''
          LANG=$1
          DAY=$2
          YEAR=''${3:-2025}

          if [ -z "$LANG" ] || [ -z "$DAY" ]; then
            echo "Usage: aoc-run <rust|java> <day> [year]"
            echo "Example: aoc-run rust 1"
            exit 1
          fi

          DAY_PADDED=$(printf "%02d" $DAY)
          DIR="$YEAR/$LANG/day-$DAY_PADDED"

          if [ ! -d "$DIR" ]; then
            echo "Error: Directory $DIR does not exist"
            exit 1
          fi

          cd "$DIR"

          case $LANG in
            rust)
              echo "Running Rust solution for day $DAY..."
              ${pkgs.cargo}/bin/cargo run --quiet
              ;;
            java)
              echo "Running Java solution for day $DAY..."
              # Compile if needed
              if [ ! -f "Solution.class" ] || [ "Solution.java" -nt "Solution.class" ]; then
                ${pkgs.jdk}/bin/javac Solution.java
              fi
              ${pkgs.jdk}/bin/java Solution
              ;;
            *)
              echo "Unknown language: $LANG"
              exit 1
              ;;
          esac
        '';

        benchSolution = pkgs.writeShellScriptBin "aoc-bench" ''
          LANG=$1
          DAY=$2
          YEAR=''${3:-2025}
          WARMUP=''${4:-3}
          RUNS=''${5:-10}

          if [ -z "$LANG" ] || [ -z "$DAY" ]; then
            echo "Usage: aoc-bench <rust|java> <day> [year] [warmup] [runs]"
            echo "Example: aoc-bench rust 1"
            echo "         aoc-bench rust 1 2025 5 20"
            exit 1
          fi

          DAY_PADDED=$(printf "%02d" $DAY)
          DIR="$YEAR/$LANG/day-$DAY_PADDED"

          if [ ! -d "$DIR" ]; then
            echo "Error: Directory $DIR does not exist"
            exit 1
          fi

          cd "$DIR"

          # Build/compile first
          case $LANG in
            rust)
              echo "Building Rust solution..."
              ${pkgs.cargo}/bin/cargo build --release --quiet
              CMD="./target/release/rust_$YEAR""_day-$DAY_PADDED"
              ;;
            java)
              echo "Compiling Java solution..."
              ${pkgs.jdk}/bin/javac Solution.java 2>/dev/null
              CMD="${pkgs.jdk}/bin/java Solution"
              ;;
            *)
              echo "Unknown language: $LANG"
              exit 1
              ;;
          esac

          echo ""
          echo "Benchmarking $LANG Day $DAY Part One..."
          ${pkgs.hyperfine}/bin/hyperfine \
            --warmup $WARMUP \
            --runs $RUNS \
            --style full \
            "$CMD part1"

          echo ""
          echo "Benchmarking $LANG Day $DAY Part Two..."
          ${pkgs.hyperfine}/bin/hyperfine \
            --warmup $WARMUP \
            --runs $RUNS \
            --style full \
            "$CMD part2"

        '';

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rustc
            cargo
            rustfmt
            clippy

            # Java
            jdk

            # Utilities
            curl
            pandoc
            hyperfine
            mkDay
            runTest
            runSolution
            benchSolution
          ];

          shellHook = ''
            echo "Advent of Code Environment"
            echo "================================"
            echo ""
            echo "Commands:"
            echo "  aoc-new-day <lang> <day>            - Create new solution from template"
            echo "  aoc-test <lang> <day> <part> [year] - Run test"
            echo "  aoc-run <lang> <day>           - Run solution (debug/dev mode)"
            echo "  aoc-bench <lang> <day>         - Benchmark a solution"
            echo ""
            echo "Examples:"
            echo "  aoc-new-day rust 1"
            echo "  aoc-run rust 1"
            echo "  aoc-run-release rust 1"
            echo "  aoc-bench java 5"
            echo ""
            echo "Setup:"
            echo "  1. Create year-specific templates: YEAR/LANG/template/"
            echo "     Example: 2025/rust/template/ and 2025/java/template/"
            echo "  2. Set AOC_SESSION='your-cookie' for input downloads"
            echo ""
            echo "Templates can use these placeholders:"
            echo "  {{DAY_PADDED}}   - Zero-padded day (e.g., 01, 15)"
            echo ""
          '';
        };

        packages.default = mkDay;
      }
    );
}
