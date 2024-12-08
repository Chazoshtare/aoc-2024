pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| parse_numbers(line))
        .map(|numbers| calculate_differences(&numbers))
        .map(|differences| {
            let none_differ_too_much = differences.iter().all(|&x| x != 0 && x < 4 && x > -4);
            let all_same_sign = differences.iter().all(|x| x.is_positive())
                || differences.iter().all(|x| x.is_negative());
            none_differ_too_much && all_same_sign
        })
        .filter(|&safe| safe)
        .count() as u32
}

pub fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| parse_numbers(line))
        .map(|numbers| calculate_differences(&numbers))
        .filter(|differences| differences.iter().filter(|&&x| x == 0).count() <= 1)
        .map(|mut differences| {
            let rising = differences_should_rise(&differences);
            let dampen_index = if rising {
                differences.iter().position(|&x| x <= 0 || x > 3)
            } else {
                differences.iter().position(|&x| x >= 0 || x < -3)
            };

            if let Some(index) = dampen_index {
                if index == 0 {
                    if is_safe_difference(differences[1], rising) {
                        differences.remove(index);
                    } else {
                        remove_and_join_with_next(&mut differences, index);
                    }
                } else if index < differences.len() - 1 {
                    let prev = differences[index - 1];
                    let next = differences[index + 1];
                    if prev + differences[index] == 0 {
                        remove_and_join_with_next(&mut differences, index);
                    } else if next + differences[index] == 0 {
                        remove_and_join_with_next(&mut differences, index - 1);
                    } else if is_safe_difference(next, rising) {
                        remove_and_join_with_next(&mut differences, index - 1);
                    } else {
                        remove_and_join_with_next(&mut differences, index)
                    }
                } else {
                    differences.remove(index);
                }
            }

            let none_differ_too_much = differences.iter().all(|&x| x != 0 && x < 4 && x > -4);
            let all_same_sign =
                differences.iter().all(|&x| x > 0) || differences.iter().all(|&x| x < 0);
            none_differ_too_much && all_same_sign
        })
        .filter(|&safe| safe)
        .count() as u32
}

fn parse_numbers(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn calculate_differences(numbers: &Vec<i32>) -> Vec<i32> {
    numbers
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<i32>>()
}

fn is_safe_difference(difference: i32, rising: bool) -> bool {
    if rising {
        difference > 0 && difference <= 3
    } else {
        difference < 0 && difference >= -3
    }
}

fn differences_should_rise(differences: &Vec<i32>) -> bool {
    let rising_count = differences.iter().filter(|&&x| x > 0).count();
    let falling_count = differences.len() - rising_count;
    rising_count > falling_count
}

fn remove_and_join_with_next(vec: &mut Vec<i32>, index: usize) {
    let old_value = vec.remove(index);
    vec[index] = vec[index] + old_value
}

#[cfg(test)]
mod tests {
    use crate::day2::{solve_part1, solve_part2};

    #[test]
    fn solves_part1_example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let solution = solve_part1(input);
        assert_eq!(solution, 2);
    }

    #[test]
    fn solves_part2_example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let solution = solve_part2(input);
        assert_eq!(solution, 4);
    }

    #[test]
    fn solves_part2_for_dampening_in_first_index() {
        //                  3  -1 -1 -2
        let input = "13 16 15 14 12";
        let result = solve_part2(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn solves_part2_for_dampening_in_last_index() {
        let input = "16 15 14 12 13";
        let result = solve_part2(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn solves_part2_for_too_big_difference() {
        let input = "30 32 35 36 50 38 39";
        let result = solve_part2(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn solves_part2_for_dampening_in_second_index_asc() {
        //                 4  -1 3  3  2  2
        let input = "8 12 11 14 17 19 21";
        //                 -8 11  3  3  2  2
        "8  0  11 14 17 19 21";
        //                 8 3  3  3  2  2
        "0 8 11 14 17 19 21";
        let result = solve_part2(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn solves_part2_for_dampening_in_second_index_desc() {
        let input = "14 17 12 10 7";
        let result = solve_part2(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn solves_part2_for_dampening_in_second_to_last_index_asc() {
        let input = "7 10 12 17 14";
        let result = solve_part2(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn solves_part2_for_dampening_in_second_to_last_index_desc() {
        let input = "13 10 8 3 6";
        let result = solve_part2(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn solves_part2_for_dampening_with_numbers_within_difference() {
        let input = "89 90 91 94 96 95 96";
        let result = solve_part2(input);
        assert_eq!(result, 1);
    }
}
