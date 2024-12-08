use crate::ws;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::{character::complete::digit1, sequence::separated_pair, IResult};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Not;

#[derive(Debug)]
struct Data {
    orders: HashMap<usize, Vec<usize>>,
    updates: Vec<Vec<usize>>,
}

pub fn day5_part1(input: &str) -> usize {
    parse(input).map_or(0, |(_, data)| compute(data))
}

pub fn day5_part2(input: &str) -> usize {
    parse(input).map_or(0, |(_, data)| compute_with_incorrect_ordered(data))
}

fn compute(Data { orders, updates }: Data) -> usize {
    updates.iter().fold(0, |acc, update| {
        acc + is_ordered(&orders, &update)
            .then_some(update[update.len() / 2])
            .unwrap_or(0)
    })
}

fn compute_with_incorrect_ordered(
    Data {
        orders,
        mut updates,
    }: Data,
) -> usize {
    updates.iter_mut().fold(0, |acc, update| {
        is_ordered(&orders, &update)
            .not()
            .then(|| {
                update.sort_by(|a, b| {
                    orders
                        .get(b)
                        .is_some_and(|ordering| ordering.contains(&a))
                        .then_some(Ordering::Greater)
                        .unwrap_or(Ordering::Less)
                });
                acc + update[update.len() / 2]
            })
            .unwrap_or(acc)
    })
}

fn is_ordered(orders: &HashMap<usize, Vec<usize>>, update: &[usize]) -> bool {
    (0..update.len()).all(|index| {
        orders
            .get(&update[index])
            .map(|ordering| {
                update[index + 1..].iter().all(|num| ordering.contains(num))
                    && update[0..index].iter().all(|num| !ordering.contains(num))
            })
            .unwrap_or(true)
    })
}

fn parse(input: &str) -> IResult<&str, Data> {
    let (rest, orders) = many1(parse_ordering)(input)?;
    let (rest, updates) = many1(parse_numbers)(rest)?;
    let orders = orders.into_iter().fold(
        HashMap::<usize, Vec<usize>>::new(),
        |mut acc, (left, right)| {
            acc.entry(left)
                .and_modify(|left| left.push(right))
                .or_insert(vec![right]);
            acc
        },
    );
    Ok((rest, Data { orders, updates }))
}

fn parse_ordering(input: &str) -> IResult<&str, (usize, usize)> {
    Ok(map(
        separated_pair(digit1, tag("|"), ws(digit1)),
        |(left, right): (&str, &str)| {
            (
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            )
        },
    )(input)?)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    Ok(map(separated_list1(tag(","), ws(digit1)), |nums| {
        nums.into_iter()
            .filter_map(|num| num.parse::<usize>().ok())
            .collect()
    })(input)?)
}

#[cfg(test)]
mod test {
    use super::{day5_part1, day5_part2};

    #[test]
    fn example_part1() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        let result = day5_part1(input);
        assert_eq!(result, 143)
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("../resources/day5_part1");
        let result = day5_part1(input);
        assert_eq!(result, 6384);
    }

    #[test]
    fn example_part2() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        let result = day5_part2(input);
        assert_eq!(result, 123)
    }

    #[test]
    fn actual_part2() {
        let input = include_str!("../resources/day5_part1");
        let result = day5_part2(input);
        assert_eq!(result, 5353);
    }
}
