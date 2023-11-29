pub fn part_one(input: &str) -> isize {
    input.chars().fold(0, |acc, brace| match brace {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

pub fn part_two(input: &str) -> isize {
    let mut position = 1;
    let mut floor = 0;
    for brace in input.chars() {
        floor = move_floor(floor, brace);
        if floor == -1 {
            break;
        }
        position += 1;
    }
    position
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
