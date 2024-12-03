use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{anychar, char, digit0},
    combinator::eof,
    multi::many_till,
    sequence::{delimited, tuple},
    IResult,
};

pub fn day3_part1(input: &str) -> usize {
    0
}

fn parse(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    let muls = many_till(parse_mul, eof)(input)?;
    println!("{:?}", muls);
    Ok((input, Vec::new()))
}

fn parse_mul(input: &str) -> IResult<&str, (&str, &str)> {
    let (rest, _) = many_till(anychar, tag("mul("))(input)?;
    let nums_parser = tuple((digit0, char(','), digit0));
    let (rest, ((left, _, right), _)) = tuple((nums_parser, tag(")")))(rest)?;
    Ok((rest, (left, right)))
}

#[cfg(test)]
mod test {
    use crate::day3::parse;

    use super::day3_part1;

    #[test]
    fn example_day3_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = day3_part1(input);
        let x = parse(input).unwrap();
        assert_eq!(result, 0)
    }
}
