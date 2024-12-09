use std::cmp::Ordering;
use std::collections::HashMap;

pub fn solve_part1(input: &str) -> u32 {
    let rules: HashMap<u32, Vec<u32>> = parse_rules(input);
    parse_pages(input)
        .iter()
        .filter(|numbers| is_sorted_by_rules(numbers, &rules))
        .map(|numbers| numbers[numbers.len() / 2])
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    let rules: HashMap<u32, Vec<u32>> = parse_rules(input);
    parse_pages(input)
        .iter_mut()
        .filter(|numbers| !is_sorted_by_rules(numbers, &rules))
        .map(|numbers| {
            sort_by_rules(numbers, &rules);
            numbers
        })
        .map(|numbers| numbers[numbers.len() / 2])
        .sum()
}

fn parse_rules(input: &str) -> HashMap<u32, Vec<u32>> {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    input
        .lines()
        .filter(|line| line.contains('|'))
        .filter_map(|line| line.split_once('|'))
        .flat_map(|numbers| {
            let first = numbers.0.parse::<u32>();
            let second = numbers.1.parse::<u32>();
            first.and_then(|a| second.map(|b| (a, b)))
        })
        .for_each(|numbers| {
            let before = numbers.0;
            let after = numbers.1;
            rules
                .entry(before)
                .and_modify(|e| e.push(after))
                .or_insert_with(|| vec![after]);
        });
    rules
}

fn parse_pages(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .filter(|line| line.contains(','))
        .map(|line| line.split(','))
        .map(|strings| strings.map(|n| n.parse::<u32>().unwrap()).collect())
        .collect()
}

fn is_sorted_by_rules(numbers: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    numbers.is_sorted_by(|a, b| {
        rules
            .get(a)
            .and_then(|pages_after| {
                if pages_after.contains(b) {
                    Some(true)
                } else {
                    Some(false)
                }
            })
            .unwrap_or(false)
    })
}

fn sort_by_rules(numbers: &mut Vec<u32>, rules: &HashMap<u32, Vec<u32>>) {
    numbers.sort_by(|a, b| {
        rules
            .get(a)
            .and_then(|pages_after| {
                if pages_after.contains(b) {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            })
            .unwrap_or(Ordering::Equal)
    })
}

#[cfg(test)]
mod tests {
    use crate::day5::{solve_part1, solve_part2};

    #[test]
    fn solves_part1_example() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let result = solve_part1(input);
        assert_eq!(result, 143);
    }

    #[test]
    fn solves_part2_example() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let result = solve_part2(input);
        assert_eq!(result, 123);
    }
}
