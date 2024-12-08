use std::iter;

pub fn solve_part1(input: &str) -> u32 {
    input
        .split("mul(")
        .skip(1)
        .filter_map(|s| s.split_once(')').map(|split| split.0))
        .filter_map(|s| s.split_once(','))
        .flat_map(|numbers| {
            let first = numbers.0.parse::<u32>();
            let second = numbers.1.parse::<u32>();
            first.and_then(|a| second.map(|b| a * b))
        })
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    let mut splitted = input.split("don't()");
    let initial_instruction = iter::once(splitted.next().unwrap());

    splitted
        .filter_map(|s| s.split_once("do()").map(|split| split.1))
        .chain(initial_instruction)
        .map(|s| solve_part1(s))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part1_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = solve_part1(input);
        assert_eq!(result, 161);
    }

    #[test]
    fn solves_part2_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = solve_part2(input);
        assert_eq!(result, 48);
    }

    #[test]
    fn solves_part2_multiple_enables() {
        let input = "don't()don't()do()do()mul(5,5)don't()";
        let result = solve_part2(input);
        assert_eq!(result, 25);
    }

    #[test]
    fn solves_part2_disables_at_start() {
        let input = "don't()abcdemul(5,5)abcde";
        let result = solve_part2(input);
        assert_eq!(result, 0);
    }
}
