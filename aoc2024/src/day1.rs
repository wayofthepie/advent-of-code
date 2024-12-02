pub fn day1_part1(input: &str) -> isize {
    let (left, right) = read_input(input);
    left.iter().zip(right).map(|(l, r)| (l - r).abs()).sum()
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
        println!("it took {}ms", duration.as_micros());
        assert_eq!(result, 1341714)
    }
}
