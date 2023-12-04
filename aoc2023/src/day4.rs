use std::collections::HashSet;

pub fn part_one(input: &str) -> usize {
    parse(input)
        .map(|matches| {
            let count = matches.len();
            if count != 0 {
                2usize.pow(count as u32 - 1)
            } else {
                0
            }
        })
        .sum::<usize>()
}

pub fn part_two(input: &str) -> usize {
    let matching = parse(input).fold(Vec::new(), |mut acc, matches| {
        acc.push(matches);
        acc
    });
    let copies = vec![1; matching.len()];
    matching
        .iter()
        .enumerate()
        .fold(copies, |mut acc, (index, matches)| {
            let times = acc[index];
            for _ in 0..times {
                acc.iter_mut()
                    .skip(index + 1)
                    .take(matches.len())
                    .for_each(|copies| *copies += 1);
            }
            acc
        })
        .into_iter()
        .sum()
}

fn parse(input: &str) -> impl Iterator<Item = HashSet<usize>> + '_ {
    input.lines().map(|line| {
        let line = parse_card(line);
        let &[winning, numbers] = line.split('|').collect::<Vec<&str>>().as_slice() else {
            unreachable!();
        };
        let winning = parse_numbers(winning);
        let numbers = parse_numbers(numbers);
        winning
            .intersection(&numbers)
            .copied()
            .collect::<HashSet<usize>>()
    })
}

fn parse_card(input: &str) -> String {
    input
        .chars()
        .skip_while(|&c| c != ':')
        .skip(1)
        .collect::<String>()
}

fn parse_numbers(input: &str) -> HashSet<usize> {
    input
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect::<HashSet<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example1() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;
        let result = part_one(input);
        assert_eq!(result, 13)
    }

    #[test]
    fn part_one_test() {
        let input = include_str!("../resources/day4_part1");
        let result = part_one(input);
        assert_eq!(result, 32001)
    }

    #[test]
    fn part_two_example1() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;
        let result = part_two(input);
        assert_eq!(result, 30)
    }

    #[test]
    fn part_two_test() {
        let input = include_str!("../resources/day4_part1");
        let result = part_two(input);
        assert_eq!(result, 5037841)
    }
}
