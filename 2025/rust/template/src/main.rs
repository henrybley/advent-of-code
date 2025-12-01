use std::env;
use std::fs;

mod part1;
mod part2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string("../../problems/day-{{DAY_PADDED}}/input.txt")
        .expect("Failed to read input file");
    
    let part = if args.len() > 1 {
        args[1].as_str()
    } else {
        "both"
    };
    
    match part {
        "1" | "part1" => {
            println!("Part 1: {}", part1::solve(&input));
        }
        "2" | "part2" => {
            println!("Part 2: {}", part2::solve(&input));
        }
        "both" | _ => {
            println!("Part 1: {}", part1::solve(&input));
            println!("Part 2: {}", part2::solve(&input));
        }
    }
}
