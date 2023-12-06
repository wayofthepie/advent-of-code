pub fn part_one_and_two(input: &str) -> usize {
    fn transform(s: &str) -> impl Iterator<Item = usize> + '_ {
        s.split_whitespace()
            .skip(1)
            .map(|s| s.parse::<usize>().unwrap())
    }
    fn compute(first: &str, second: &str) -> usize {
        transform(first)
            .zip(transform(second))
            .map(|(time, distance)| {
                (0..time)
                    .filter(|&button| (time - button) * button > distance)
                    .count()
            })
            .product()
    }
    let mut lines = input.lines();
    if let (Some(first), Some(second)) = (lines.next(), lines.next()) {
        compute(first, second)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example1() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        let result = part_one_and_two(input);
        assert_eq!(result, 288);
    }

    #[test]
    fn part_one_test() {
        let input = include_str!("../resources/day6_part1");
        let result = part_one_and_two(input);
        assert_eq!(result, 1312850);
    }

    #[test]
    fn part_two_test() {
        let input = r#"Time:        48938466
Distance:   261119210191063"#;
        let result = part_one_and_two(input);
        assert_eq!(result, 36749103);
    }
}
