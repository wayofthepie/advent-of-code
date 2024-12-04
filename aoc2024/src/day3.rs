use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char, digit0},
    multi::{many0, many_till},
    sequence::{delimited, tuple},
    IResult,
};

pub fn day3_part1(input: &str) -> usize {
    parse(input).map_or_else(|_| 0, |(_, muls)| muls.sum())
}

pub fn day3_part2(input: &str) -> usize {
    0
}

fn parse<'a>(input: &'a str) -> IResult<&'a str, impl Iterator<Item = usize> + 'a> {
    let (rest, muls) = many0(parse_mul)(input)?;
    let muls = muls
        .into_iter()
        .flat_map(|(left, right)| left.parse::<usize>().ok().zip(right.parse::<usize>().ok()))
        .map(|(left, right)| left * right);
    Ok((rest, muls))
}

fn parse_mul(input: &str) -> IResult<&str, (&str, &str)> {
    let (rest, (_, (_, (left, _, right)))) = many_till(
        anychar,
        tuple((
            tag("mul"),
            delimited(tag("("), tuple((digit0, char(','), digit0)), tag(")")),
        )),
    )(input)?;
    Ok((rest, (left, right)))
}

#[cfg(test)]
mod test {
    use super::{day3_part1, day3_part2};

    #[test]
    fn example_day3_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = day3_part1(input);
        assert_eq!(result, 161)
    }

    #[test]
    fn actual_day3_part1() {
        let input = include_str!("../resources/day3_part1");
        let result = day3_part1(input);
        assert_eq!(result, 161)
    }

    #[test]
    fn example_day3_part2() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = day3_part2(input);
        assert_eq!(result, 161)
    }
}
