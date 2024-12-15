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

pub fn solve_part2(input: &str) -> usize {
    let disk_map = parse_disk_map_to_blocks(input);
    let defragmented = defragment(disk_map);
    let mut checksum = 0;
    let mut position_index = 0;
    for block in defragmented {
        checksum += block.calculate_checksum(position_index);
        position_index += block.size;
    }
    checksum
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

fn parse_disk_map_to_blocks(input: &str) -> Vec<DiskBlock> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .filter_map(|(i, n)| {
            if i % 2 == 0 {
                Some(DiskBlock {
                    id: i / 2,
                    occupied: true,
                    size: n,
                })
            } else if n > 0 {
                Some(DiskBlock::create_empty(n))
            } else {
                None
            }
        })
        .collect()
}

fn defragment(mut blocks: Vec<DiskBlock>) -> Vec<DiskBlock> {
    let mut defragmented: Vec<DiskBlock> = vec![];
    while !blocks.is_empty() {
        let block = blocks.remove(0);
        if block.occupied {
            defragmented.push(block);
        } else {
            let mut free_space_size = block.size;
            while free_space_size > 0 {
                let movable_index = blocks
                    .iter()
                    .rposition(|b| b.size <= free_space_size && b.occupied);
                match movable_index {
                    Some(index) => {
                        let movable_block = blocks.remove(index);
                        blocks.insert(index, movable_block.as_empty());
                        free_space_size -= movable_block.size;
                        defragmented.push(movable_block);
                    }
                    None => {
                        let leftover = DiskBlock::create_empty(free_space_size);
                        defragmented.push(leftover);
                        break;
                    }
                }
            }
        }
    }
    defragmented
}

#[derive(Debug)]
struct DiskBlock {
    id: usize,
    occupied: bool,
    size: u32,
}

impl DiskBlock {
    fn create_empty(size: u32) -> Self {
        DiskBlock {
            id: 0,
            occupied: false,
            size,
        }
    }

    fn as_empty(&self) -> Self {
        DiskBlock {
            id: 0,
            occupied: false,
            size: self.size,
        }
    }

    fn calculate_checksum(&self, position_index: u32) -> usize {
        if self.occupied {
            let mut checksum = 0;
            for i in 0..self.size {
                checksum += (i + position_index) as usize * self.id
            }
            checksum
        } else {
            0
        }
    }
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
    fn calculates_block_checksum() {
        let disk_block = DiskBlock {
            id: 5,
            occupied: true,
            size: 3,
        };
        let checksum = disk_block.calculate_checksum(2);
        assert_eq!(checksum, 10 + 15 + 20);
    }
}
