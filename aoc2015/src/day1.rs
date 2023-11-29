pub fn part_one(input: &str) -> isize {
    input.chars().fold(0, move_floor)
}

pub fn part_two(input: &str) -> isize {
    let mut floor = 0;
    for (position, brace) in input.chars().enumerate() {
        floor = move_floor(floor, brace);
        if floor == -1 {
            return position as isize + 1;
        }
    }
    0
}

fn move_floor(floor: isize, brace: char) -> isize {
    match brace {
        '(' => floor + 1,
        ')' => floor - 1,
        _ => floor,
    }
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

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
    fn test_part1() {
        let input = include_str!("../resources/day1_part1");
        let result = part_one(input);
        assert_eq!(result, 280);
    }

    #[test]
    fn example1_part2() {
        let input = ")";
        let result = part_two(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn example2_part2() {
        let input = "()())";
        let result = part_two(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../resources/day1_part1");
        let result = part_two(input);
        assert_eq!(result, 1797);
    }
}
