use std::fs;
use std::path::Path;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let day1_input = read_input(Path::new("inputs/day1.txt"));
    let solution1_1 = day1::solve_part1(&day1_input);
    println!("Day 1, part 1 solution: {}", solution1_1);
    let solution1_2 = day1::solve_part2(&day1_input);
    println!("Day 1, part 2 solution: {}", solution1_2);

    let day2_input = read_input(Path::new("inputs/day2.txt"));
    let solution2_1 = day2::solve_part1(&day2_input);
    println!("Day 2, part 1 solution: {}", solution2_1);
    let solution2_2 = day2::solve_part2(&day2_input);
    println!("Day 2, part 2 solution: {}", solution2_2);

    let day3_input = read_input(Path::new("inputs/day3.txt"));
    let solution3_1 = day3::solve_part1(&day3_input);
    println!("Day 3, part 1 solution: {}", solution3_1);
    let solution3_2 = day3::solve_part2(&day3_input);
    println!("Day 3, part 2 solution: {}", solution3_2);

    let day4_input = read_input(Path::new("inputs/day4.txt"));
    let solution4_1 = day4::solve_part1(&day4_input);
    println!("Day 4, part 1 solution: {}", solution4_1);
    let solution4_2 = day4::solve_part2(&day4_input);
    println!("Day 4, part 2 solution: {}", solution4_2);

    let day5_input = read_input(Path::new("inputs/day5.txt"));
    let solution5_1 = day5::solve_part1(&day5_input);
    println!("Day 5, part 1 solution: {}", solution5_1);
    let solution5_2 = day5::solve_part2(&day5_input);
    println!("Day 5, part 2 solution: {}", solution5_2);

    let day6_input = read_input(Path::new("inputs/day6.txt"));
    let solution6_1 = day6::solve_part1(&day6_input);
    println!("Day 6, part 1 solution: {}", solution6_1);
    let solution6_2 = day6::solve_part2(&day6_input);
    println!("Day 6, part 2 solution: {}", solution6_2);

    let day7_input = read_input(Path::new("inputs/day7.txt"));
    let solution7_1 = day7::solve_part1(&day7_input);
    println!("Day 7, part 1 solution: {}", solution7_1);
    let solution7_2 = day7::solve_part2(&day7_input);
    println!("Day 7, part 2 solution: {}", solution7_2);
}

fn read_input(path: &Path) -> String {
    fs::read_to_string(path).expect(format!("couldn't read input file {:?}", path).as_str())
}
