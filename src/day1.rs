use std::fs;
use std::path::Path;

pub fn solve_part1(input_path: &Path) -> u32 {
    let (mut left_list, mut right_list) = parse_input(&read_input(input_path));
    left_list.sort();
    right_list.sort();

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

pub fn solve_part2(input_path: &Path) -> u32 {
    let (left_list, right_list) = parse_input(&read_input(input_path));

    left_list.iter()
        .map(|number| {
            let occurrences = right_list.iter().filter(|x| *x == number).count();
            number * occurrences as u32
        })
        .sum()
}

fn parse_input(string: &String) -> (Vec<u32>, Vec<u32>) {
    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];

    string
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let mut numbers = line.split_whitespace();
            left_list.push(numbers.next().unwrap().parse::<u32>().unwrap());
            right_list.push(numbers.next().unwrap().parse::<u32>().unwrap());
        });

    (left_list, right_list)
}

fn read_input(path: &Path) -> String {
    fs::read_to_string(path).expect(format!("couldn't read input file {:?}", path).as_str())
}
