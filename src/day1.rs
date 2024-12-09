use std::collections::HashMap;

pub fn solve_part1(input: &str) -> u32 {
    let (mut left_list, mut right_list) = parse_input(input);
    left_list.sort();
    right_list.sort();

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    let (left_list, right_list) = parse_input(input);
    let mut occurrences: HashMap<u32, u32> = HashMap::new();
    right_list.iter()
        .for_each(|&number| {
            occurrences.entry(number)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

    left_list
        .iter()
        .map(|number| {
            let occurrences = occurrences.get(number).unwrap_or(&0);
            number * occurrences
        })
        .sum()
}

fn parse_input(string: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];

    string
        .lines()
        .for_each(|line| {
            let mut numbers = line.split_whitespace();
            left_list.push(numbers.next().unwrap().parse::<u32>().unwrap());
            right_list.push(numbers.next().unwrap().parse::<u32>().unwrap());
        });

    (left_list, right_list)
}

#[cfg(test)]
mod tests {
    use crate::day1::{solve_part1, solve_part2};

    #[test]
    fn solves_part1_example() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let solution = solve_part1(input);
        assert_eq!(solution, 11);
    }

    #[test]
    fn solves_part2_example() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let solution = solve_part2(input);
        assert_eq!(solution, 31);
    }
}
