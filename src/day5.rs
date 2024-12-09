use std::collections::HashMap;

pub fn solve_part1(input: &str) -> u32 {
    let rules: HashMap<u32, Vec<u32>> = parse_rules(input);
    input
        .lines()
        .filter(|line| line.contains(','))
        .map(|line| line.split(','))
        .map(|strings| strings.map(|n| n.parse::<u32>().unwrap()))
        .map(|numbers| {
            numbers
                .enumerate()
                .map(|(index, number)| (number, index))
                .collect::<HashMap<_, _>>()
        })
        .filter(|indexed| obeys_rules(indexed, &rules))
        .map(|indexed| get_middle(&indexed))
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    0
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

fn obeys_rules(indexed_numbers: &HashMap<u32, usize>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    indexed_numbers.iter().all(|(number, index)| {
        rules
            .get(number)
            .and_then(|pages_after| {
                let valid = pages_after.iter().all(|page_after| {
                    let page_after_index = indexed_numbers.get(page_after).unwrap_or(&9999usize);
                    index < page_after_index
                });
                Some(valid)
            })
            .unwrap_or(true)
    })
}

fn get_middle(indexed_numbers: &HashMap<u32, usize>) -> u32 {
    let length = indexed_numbers.len();
    indexed_numbers
        .iter()
        .filter(|&(&number, &index)| index == length / 2)
        .map(|(&number, &index)| number)
        .next()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::day5::solve_part1;

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
}
