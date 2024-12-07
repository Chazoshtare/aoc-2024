use std::path::Path;

mod day1;

fn main() {
    let solution1_1 = day1::solve_part1(Path::new("inputs/day1.txt"));
    println!("Day 1, part 1 solution: {}", solution1_1);

    let solution1_2 = day1::solve_part2(Path::new("inputs/day1.txt"));
    println!("Day 1, part 2 solution: {}", solution1_2);
}
