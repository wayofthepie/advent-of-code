use std::collections::HashMap;

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|ch| {
                ch.is_ascii_digit()
                    .then(|| ch.to_digit(10).unwrap() as usize)
            });
            let first = digits.next().unwrap_or(0);
            let last = digits.last().unwrap_or(first);
            (first * 10) + last
        })
        .sum()
}

const UNKNOWN: usize = 10;
const NUMBER_STRINGS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_two(input: &str) -> usize {
    let numbers_map = NUMBER_STRINGS
        .iter()
        .enumerate()
        .map(|(index, &num_str)| (num_str.to_owned(), index))
        .collect();
    input
        .lines()
        .map(|line| {
            let (first, last) = find_numbers(line, &NUMBER_STRINGS, &numbers_map);
            (first * 10) + last
        })
        .sum()
}

fn find_numbers(
    line: &str,
    numbers_as_str: &[&str],
    number_map: &HashMap<String, usize>,
) -> (usize, usize) {
    let mut number_positions = vec![UNKNOWN; line.len()];
    numbers_as_str
        .iter()
        .enumerate()
        .for_each(|(position, &num_str)| {
            line.match_indices(num_str).for_each(|(index, _)| {
                number_positions[index] = number_map.get(num_str).copied().unwrap_or(UNKNOWN)
            });
            line.match_indices(&position.to_string())
                .for_each(|(index, _)| number_positions[index] = position);
        });
    let mut final_nums_iter = number_positions.iter().copied().filter(|&n| n != UNKNOWN);
    let first = final_nums_iter.next().unwrap_or(0);
    let last = final_nums_iter.last().unwrap_or(first);
    (first, last)
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn part_one_example1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;
        let result = part_one(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("../resources/day1_part1");
        let result = part_one(input);
        assert_eq!(result, 54632);
    }

    #[test]
    fn part_two_example1() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        let result = part_two(input);
        assert_eq!(result, 281);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../resources/day1_part1");
        let result = part_two(input);
        assert_eq!(result, 54019);
    }
}
