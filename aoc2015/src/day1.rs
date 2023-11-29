pub fn part_one(input: &str) -> isize {
    input.chars().fold(0, |acc, brace| match brace {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

#[cfg(test)]
mod test {
    use super::part_one;

    #[test]
    fn example1() {
        let input = "(())";
        let result = part_one(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn example2() {
        let input = "(()(()(";
        let result = part_one(input);
        assert_eq!(result, 3)
    }

    #[test]
    fn example3() {
        let input = "))(((((";
        let result = part_one(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("../resources/day1_part1");
        let result = part_one(input);
        assert_eq!(result, 280);
    }
}
