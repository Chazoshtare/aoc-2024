use std::iter::repeat_n;

pub fn solve_part1(input: &str) -> usize {
    let disk_map = parse_disk_map(input);
    let last_index = disk_map.len() - 1;
    let mut back_indexer = last_index;
    let mut checksum = 0;
    for i in 0..last_index {
        if i > back_indexer {
            break;
        }
        match disk_map[i] {
            Some(id) => checksum += i * id,
            None => {
                while disk_map[back_indexer] == None {
                    back_indexer -= 1;
                }
                checksum += i * disk_map[back_indexer].unwrap();
                back_indexer -= 1;
            }
        }
    }
    checksum
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
}
