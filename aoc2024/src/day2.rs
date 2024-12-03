pub fn day2_part1(input: &str) -> usize {
    let mut safe = 0;
    for line in input.lines() {
        let nums: Vec<isize> = line
            .split(" ")
            .filter_map(|s| s.parse::<isize>().ok())
            .collect();
        if is_safe(&nums) {
            safe += 1;
        }
    }
    safe
}

pub fn day2_part2(input: &str) -> usize {
    let mut safe = 0;
    for line in input.lines() {
        let nums: Vec<isize> = line
            .split(" ")
            .filter_map(|s| s.parse::<isize>().ok())
            .collect();
        if is_safe(&nums) {
            safe += 1;
        } else {
            for index in 0..nums.len() {
                let safety: Vec<isize> = nums
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != index)
                    .map(|(_, v)| *v)
                    .collect();
                if is_safe(&safety) {
                    safe += 1;
                    break;
                }
            }
        }
    }
    safe
}

fn is_safe(nums: &[isize]) -> bool {
    let diffs: Vec<isize> = nums.windows(2).map(|nums| nums[0] - nums[1]).collect();
    let increasing = diffs.iter().all(|num| num.is_positive());
    let decreasing = diffs.iter().all(|num| num.is_negative());
    if increasing || decreasing {
        for num in diffs {
            if num.abs() < 1 || num.abs() > 3 {
                return false;
            }
        }
        return true;
    }
    false
}

#[cfg(test)]
mod test {
    use super::{day2_part1, day2_part2};
    use std::time::SystemTime;

    #[test]
    fn example_day2_part1() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        let result = day2_part1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn actual_day2_part1() {
        let input = include_str!("../resources/day2_part1");
        let start = SystemTime::now();
        let result = day2_part1(input);
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        println!("it took {}µs", duration.as_micros());
        assert_eq!(result, 411)
    }

    #[test]
    fn example_day2_part2() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        let result = day2_part2(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn actual_day2_part2() {
        let input = include_str!("../resources/day2_part1");
        let start = SystemTime::now();
        let result = day2_part2(input);
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        println!("it took {}µs", duration.as_micros());
        assert_eq!(result, 465)
    }
}
