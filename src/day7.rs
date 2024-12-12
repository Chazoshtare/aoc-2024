pub fn solve_part1(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .filter(|(result, numbers)| can_evaluate_to_result(numbers, *result, false))
        .map(|(result, _)| result)
        .sum()
}

pub fn solve_part2(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .filter(|(result, numbers)| can_evaluate_to_result(numbers, *result, true))
        .map(|(result, _)| result)
        .sum()
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(result, numbers)| {
            (
                result.parse::<u64>().unwrap(),
                numbers
                    .split_whitespace()
                    .rev()
                    .map(|number| number.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .collect()
}

fn can_evaluate_to_result(reversed_numbers: &[u64], result: u64, with_concatenation: bool) -> bool {
    let number = reversed_numbers[0];
    if reversed_numbers.len() == 1 {
        return number == result;
    }
    if with_concatenation {
        let result_string = result.to_string();
        let number_string = number.to_string();
        if result_string.ends_with(number_string.as_str()) {
            let new_result = result_string
                .strip_suffix(number_string.as_str())
                .unwrap()
                .parse::<u64>()
                .unwrap_or(0);
            if can_evaluate_to_result(&reversed_numbers[1..], new_result, with_concatenation) {
                return true;
            }
        }
    }
    let is_divisible = result % number == 0;
    if is_divisible {
        let new_result = result / number;
        if can_evaluate_to_result(&reversed_numbers[1..], new_result, with_concatenation) {
            return true;
        }
    }
    if result > number {
        let new_result = result - number;
        if can_evaluate_to_result(&reversed_numbers[1..], new_result, with_concatenation) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part1_example() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
        let result = solve_part1(input);
        assert_eq!(result, 3749);
    }

    #[test]
    fn solves_part2_example() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
        let result = solve_part2(input);
        assert_eq!(result, 11387);
    }
}
