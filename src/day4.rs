pub fn solve_part1(input: &str) -> usize {
    let char_matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut searchables: Vec<String> = vec![];

    searchables.append(&mut get_horizontal_strings(&char_matrix));
    searchables.append(&mut get_vertical_strings(&char_matrix));
    searchables.append(&mut get_diagonal_strings(&char_matrix));

    let xmas_count: usize = searchables.iter().map(|s| s.matches("XMAS").count()).sum();
    let samx_count: usize = searchables.iter().map(|s| s.matches("SAMX").count()).sum();

    xmas_count + samx_count
}

pub fn solve_part2(input: &str) -> u32 {
    let char_matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let length = char_matrix.len();
    let mut hits = 0;

    for i in 1..length - 1 {
        for j in 1..length - 1 {
            let middle = char_matrix[i][j];
            if middle == 'A' {
                let left_diagonal = format!(
                    "{}{}{}",
                    char_matrix[i - 1][j - 1],
                    'A',
                    char_matrix[i + 1][j + 1]
                );
                let right_diagonal = format!(
                    "{}{}{}",
                    char_matrix[i - 1][j + 1],
                    'A',
                    char_matrix[i + 1][j - 1]
                );
                if (left_diagonal == "MAS" || left_diagonal == "SAM") &&
                    (right_diagonal == "MAS" || right_diagonal == "SAM") {
                    hits = hits + 1;
                }
            }
        }
    }
    hits
}

fn get_horizontal_strings(char_matrix: &Vec<Vec<char>>) -> Vec<String> {
    char_matrix
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect()
}

fn get_vertical_strings(char_matrix: &Vec<Vec<char>>) -> Vec<String> {
    let length = char_matrix.len();
    let mut strings: Vec<String> = vec![];
    for i in 0..length {
        let mut searchable = String::new();
        for j in 0..length {
            searchable.push(char_matrix[j][i])
        }
        strings.push(searchable);
    }
    strings
}

fn get_diagonal_strings(char_matrix: &Vec<Vec<char>>) -> Vec<String> {
    let length = char_matrix.len();
    let mut strings: Vec<String> = vec![];
    for i in 3..length {
        let mut upper_left = String::new();
        let mut upper_right = String::new();
        let mut lower_left = String::new();
        let mut lower_right = String::new();
        let mut row = i;
        let mut column = 0;
        loop {
            upper_left.push(char_matrix[row][column]);
            upper_right.push(char_matrix[row][length - 1 - column]);
            lower_right.push(char_matrix[length - 1 - row][length - 1 - column]);
            lower_left.push(char_matrix[length - 1 - row][column]);
            if row != 0 {
                row -= 1;
                column += 1;
            } else {
                break;
            }
        }
        strings.push(upper_left);
        strings.push(upper_right);
        if i != length - 1 {
            strings.push(lower_left);
            strings.push(lower_right);
        }
    }
    strings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part1_example() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = solve_part1(input);
        assert_eq!(result, 18);
    }

    #[test]
    fn solves_part2_example() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = solve_part2(input);
        assert_eq!(result, 9);
    }

    #[test]
    fn solves_part2_for_one_cross() {
        let input = "M.M
.A.
S.S";
        let result = solve_part2(input);
        assert_eq!(result, 1);
    }
}
