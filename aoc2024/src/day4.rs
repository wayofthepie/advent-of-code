pub fn day4_part1(input: &str) -> usize {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let positions = |row: usize, col: usize| {
        let row = row as isize;
        let col = col as isize;
        [
            vec![(row, col), (row, col + 1), (row, col + 2), (row, col + 3)],
            vec![(row, col), (row + 1, col), (row + 2, col), (row + 3, col)],
            vec![
                (row, col),
                (row + 1, col - 1),
                (row + 2, col - 2),
                (row + 3, col - 3),
            ],
            vec![
                (row, col),
                (row + 1, col + 1),
                (row + 2, col + 2),
                (row + 3, col + 3),
            ],
        ]
    };
    let mut sum = 0;
    for row in 0..data.len() {
        for col in 0..data.len() {
            sum += positions(row, col)
                .iter()
                .fold(0, |acc, position| acc + count(&data, &position));
        }
    }
    sum
}

pub fn day4_part2(input: &str) -> usize {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    for row in 0..data.len() as isize {
        for col in 0..data[0].len() as isize {
            if is_chr(&data, row + 1, col + 1, 'A')
                && (is_chr(&data, row, col, 'S') && is_chr(&data, row + 2, col + 2, 'M')
                    || is_chr(&data, row, col, 'M') && is_chr(&data, row + 2, col + 2, 'S'))
                && (is_chr(&data, row + 2, col, 'S') && is_chr(&data, row, col + 2, 'M')
                    || is_chr(&data, row + 2, col, 'M') && is_chr(&data, row, col + 2, 'S'))
            {
                sum += 1
            }
        }
    }
    sum
}

fn is_chr(data: &[Vec<char>], row: isize, col: isize, chr: char) -> bool {
    if row.is_negative() || col.is_negative() {
        return false;
    }
    let is_given_char = |c: &char| *c == chr;
    data.get(row as usize)
        .is_some_and(|row| row.get(col as usize).is_some_and(is_given_char))
}

fn count(data: &[Vec<char>], positions: &[(isize, isize)]) -> usize {
    let xmas: Vec<char> = "XMAS".chars().collect();
    let samx: Vec<char> = "SAMX".chars().collect();
    let (has_xmas, has_samx) = positions.iter().enumerate().fold(
        (true, true),
        |(has_xmas, has_samx), (index, (row, col))| {
            (
                has_xmas && is_chr(&data, *row, *col, xmas[index]),
                has_samx && is_chr(&data, *row, *col, samx[index]),
            )
        },
    );
    has_xmas as usize + has_samx as usize
}

#[cfg(test)]
mod test {
    use crate::day4::{day4_part1, day4_part2};

    #[test]
    fn example_day4_part1() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let result = day4_part1(input);
        assert_eq!(result, 18)
    }

    #[test]
    fn actual_day4_part1() {
        let input = include_str!("../resources/day4_part1").trim();
        let result = day4_part1(input);
        assert_eq!(result, 2618)
    }

    #[test]
    fn example_day4_part2() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let result = day4_part2(input);
        assert_eq!(result, 9)
    }

    #[test]
    fn actual_day4_part2() {
        let input = include_str!("../resources/day4_part1").trim();
        let result = day4_part2(input);
        assert_eq!(result, 2011)
    }
}
