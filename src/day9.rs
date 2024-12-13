use std::iter::repeat_n;

pub fn solve_part1(input: &str) -> usize {
    let mut disk_map = parse_disk_map(input);
    reposition_to_free_space(&mut disk_map);
    calculate_checksum(&disk_map)
}

pub fn solve_part2(input: &str) -> u32 {
    0
}

fn parse_disk_map(input: &str) -> Vec<Option<usize>> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .flat_map(|(i, n)| {
            if i % 2 == 0 {
                repeat_n(Some(i / 2), n as usize)
            } else {
                repeat_n(None, n as usize)
            }
        })
        .collect()
}

fn reposition_to_free_space(disk_map: &mut Vec<Option<usize>>) {
    for i in 0..disk_map.len() {
        let mut element = disk_map[i];
        if element.is_none() {
            disk_map[i] = get_last_present(&mut disk_map[i..])
        }
    }
}

fn get_last_present(disk_slice: &mut [Option<usize>]) -> Option<usize> {
    disk_slice
        .iter_mut()
        .rfind(|n| n.is_some())
        .map(|mut n| n.take())
        .flatten()
}

fn calculate_checksum(disk_map: &[Option<usize>]) -> usize {
    disk_map
        .iter()
        .filter_map(|n| *n)
        .enumerate()
        .fold(0, |acc, (i, n)| acc + i * n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part1_example() {
        let input = "2333133121414131402";
        let result = solve_part1(input);
        assert_eq!(result, 1928);
    }

    #[test]
    fn solves_part2_example() {
        let input = "2333133121414131402";
        let result = solve_part2(input);
        assert_eq!(result, 2858);
    }

    #[test]
    fn parses_disk_map() {
        let input = "20113";
        let result = parse_disk_map(input);
        let expected = vec![Some(0), Some(0), Some(1), None, Some(2), Some(2), Some(2)];
        assert_eq!(result, expected);
    }

    #[test]
    fn calculates_checksum() {
        let input = vec![Some(0), Some(0), Some(1), Some(8), Some(2), Some(7)];
        let result = calculate_checksum(&input);
        assert_eq!(result, 2 + 24 + 8 + 35);
    }
}
