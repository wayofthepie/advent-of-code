use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::eof,
    multi::{many_till, separated_list1},
    sequence::tuple,
    IResult,
};
use rayon::prelude::*;

use crate::ws;

type MappingRange = Vec<Vec<Vec<usize>>>;

#[derive(Debug, Clone, Copy, PartialEq)]
struct RangeMap {
    src: isize,
    dest: isize,
    len: isize,
}

impl RangeMap {
    fn new(src: isize, dest: isize, len: isize) -> Self {
        Self { src, dest, len }
    }
}

pub fn part_one(input: &str) -> usize {
    let (seeds, steps) = seeds_and_steps(input);
    seeds.into_iter().fold(usize::MAX, |acc, seed| {
        let n = steps.iter().fold(seed as isize, |acc, step| {
            step.iter()
                .find(|range| acc >= range.src && acc < range.src + range.len)
                .map(|range| {
                    let diff = range.dest - range.src;
                    acc + diff
                })
                .unwrap_or(acc)
        });
        acc.min(n as usize)
    })
}

pub fn part_two(input: &str) -> usize {
    let (seeds, steps) = seeds_and_steps(input);
    seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .par_bridge()
        .flat_map(|(start, len)| (start..start + len))
        .fold(|| usize::MAX, fold_seeds(&steps))
        .min()
        .unwrap_or(0)
}

fn fold_seeds(steps: &[Vec<RangeMap>]) -> impl Fn(usize, usize) -> usize + '_ {
    |acc, seed| {
        let n = steps.iter().fold(seed as isize, |acc, step| {
            step.iter()
                .find(|range| acc >= range.src && acc < range.src + range.len)
                .map(|range| {
                    let diff = range.dest - range.src;
                    acc + diff
                })
                .unwrap_or(acc)
        });
        acc.min(n as usize)
    }
}

fn seeds_and_steps(input: &str) -> (Vec<usize>, Vec<Vec<RangeMap>>) {
    let (_, (seeds, mappings)) = parse(input).unwrap();
    let steps = mappings
        .into_iter()
        .map(|values| {
            values
                .into_iter()
                .map(|range| {
                    let &[dest, src, len] = range.as_slice() else {
                        panic!();
                    };
                    RangeMap::new(src as isize, dest as isize, len as isize)
                })
                .collect()
        })
        .collect();
    (seeds, steps)
}

fn parse(input: &str) -> IResult<&str, (Vec<usize>, MappingRange)> {
    let (rest, (seeds, (mappings, _))) =
        tuple((parse_seeds, many_till(ws(parse_ranges), eof)))(input)?;
    Ok((rest, (seeds, mappings)))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
    let (rest, _) = tag("seeds: ")(input)?;
    let (rest, nums) = separated_list1(tag(" "), digit1)(rest)?;
    let nums = nums
        .into_iter()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
    Ok((rest, nums))
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    let parse_ids = alt((
        tag("seed-to-soil"),
        tag("soil-to-fertilizer"),
        tag("fertilizer-to-water"),
        tag("water-to-light"),
        tag("light-to-temperature"),
        tag("temperature-to-humidity"),
        tag("humidity-to-location"),
    ));
    let (rest, (_, _, ranges)) = tuple((
        parse_ids,
        ws(tag("map:")),
        separated_list1(newline, separated_list1(space1, digit1)),
    ))(input)?;
    let ranges = ranges
        .into_iter()
        .map(|entry| {
            entry
                .into_iter()
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    Ok((rest, ranges))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example1() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
        let result = part_one(input);
        assert_eq!(result, 35);
    }

    #[test]
    fn part_one_test() {
        let input = include_str!("../resources/day5_part1");
        let result = part_one(input);
        assert_eq!(result, 836040384);
    }

    #[test]
    fn part_two_example1() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
        let result = part_two(input);
        assert_eq!(result, 46);
    }

    #[test]
    fn part_two_test() {
        let input = include_str!("../resources/day5_part1");
        let result = part_two(input);
        assert_eq!(result, 10834440);
    }
}
