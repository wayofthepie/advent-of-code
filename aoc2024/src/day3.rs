use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit0},
    combinator::map,
    multi::{many0, many_till},
    sequence::{delimited, tuple},
    IResult,
};

enum Token {
    Mul(Mul),
    Do,
    Dont,
}

struct Mul {
    left: usize,
    right: usize,
}

pub fn day3_part1(input: &str) -> usize {
    parse(input).map_or_else(
        |_| 0,
        |(_, tokens)| {
            tokens
                .into_iter()
                .map(|token| match token {
                    Token::Mul(Mul { left, right }) => left * right,
                    _ => 0,
                })
                .sum()
        },
    )
}

pub fn day3_part2(input: &str) -> usize {
    parse(input).map_or_else(
        |_| 0,
        |(_, tokens)| {
            tokens
                .into_iter()
                .fold((true, 0), |(should_do, sum), token| match token {
                    Token::Mul(Mul { left, right }) => (
                        should_do,
                        should_do.then(|| sum + left * right).unwrap_or(sum),
                    ),
                    Token::Do => (true, sum),
                    Token::Dont => (false, sum),
                })
                .1
        },
    )
}

fn parse(input: &str) -> IResult<&str, Vec<Token>> {
    let parse_do = map(tag("do()"), |_| Token::Do);
    let parse_dont = map(tag("don't()"), |_| Token::Dont);
    let parse_muls = map(
        many_till(anychar, alt((parse_mul_instruction, parse_do, parse_dont))),
        |(_, token)| token,
    );
    Ok(many0(parse_muls)(input)?)
}

fn parse_mul_instruction(input: &str) -> IResult<&str, Token> {
    let (rest, (_, (left, _, right))) = tuple((
        tag("mul"),
        delimited(tag("("), tuple((digit0, char(','), digit0)), tag(")")),
    ))(input)?;
    Ok((
        rest,
        Token::Mul(Mul {
            left: left.parse::<usize>().unwrap(),
            right: right.parse::<usize>().unwrap(),
        }),
    ))
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
        assert_eq!(result, 170807108)
    }

    #[test]
    fn example_day3_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = day3_part2(input);
        assert_eq!(result, 48)
    }

    #[test]
    fn actual_day3_part2() {
        let input = include_str!("../resources/day3_part1");
        let result = day3_part2(input);
        assert_eq!(result, 74838033)
    }
}
