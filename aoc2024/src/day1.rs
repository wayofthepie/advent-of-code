use std::collections::HashMap;

pub fn day1_part1(input: &str) -> isize {
    let (left, right) = read_input(input);
    left.iter().zip(right).map(|(l, r)| (l - r).abs()).sum()
}

pub fn day1_part2(input: &str) -> isize {
    let (left, right) = read_input(input);
    let frequency = right
        .iter()
        .fold(HashMap::<isize, usize>::new(), |mut acc, num| {
            acc.entry(*num)
                .and_modify(|num| {
                    *num += 1;
                })
                .or_insert(1);
            acc
        });

    let mut total = 0;
    for left_num in left.iter() {
        if let Some(&count) = frequency.get(left_num) {
            total += left_num * count as isize;
        }
    }
    total
}

fn read_input(input: &str) -> (Vec<isize>, Vec<isize>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let split: Vec<isize> = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        left.push(*split.get(0).unwrap());
        right.push(*split.get(1).unwrap());
    }
    left.sort();
    right.sort();
    (left, right)
}

#[cfg(test)]
mod test {
    use std::time::SystemTime;

    use crate::day1::day1_part2;

    use super::day1_part1;

    #[test]
    fn example_day1_part1() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        let result = day1_part1(input);
        assert_eq!(result, 11)
    }

    #[test]
    fn actual_day1_part1() {
        let input = include_str!("../resources/day1_part1");
        let start = SystemTime::now();
        let result = day1_part1(input);
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        println!("it took {}µs", duration.as_micros());
        assert_eq!(result, 1341714)
    }

    #[test]
    fn example_day1_part2() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        let result = day1_part2(input);
        assert_eq!(result, 31)
    }

    #[test]
    fn actual_day1_part2() {
        let input = include_str!("../resources/day1_part1");
        let start = SystemTime::now();
        let result = day1_part2(input);
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        println!("it took {}µs", duration.as_micros());
        assert_eq!(result, 27384707)
    }
}
