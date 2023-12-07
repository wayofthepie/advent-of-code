use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, multispace0},
    combinator::{eof, opt},
    error::ParseError,
    multi::{many0, many_till},
    sequence::{delimited, terminated, tuple},
    IResult,
};

#[derive(Debug)]
struct Game {
    id: usize,
    matches: Vec<Vec<(usize, Color)>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Blue,
    Green,
    Red,
}

impl FromStr for Color {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Color::Blue),
            "green" => Ok(Color::Green),
            "red" => Ok(Color::Red),
            _ => Err("Unexpected color!".into()),
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let (_, game) = parse_game(line).unwrap();
        if game
            .matches
            .iter()
            .all(|r#match| r#match.iter().all(is_possible))
        {
            sum += game.id
        }
    }
    sum
}

#[derive(Default, Debug)]
struct MaxTracker {
    blue: usize,
    green: usize,
    red: usize,
}

pub fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (_, game) = parse_game(line).unwrap();
            let tracker = game
                .matches
                .into_iter()
                .fold(MaxTracker::default(), |acc, r#match| {
                    r#match.into_iter().fold(acc, |mut acc, round| {
                        match round {
                            (num, Color::Blue) => acc.blue = acc.blue.max(num),
                            (num, Color::Green) => acc.green = acc.green.max(num),
                            (num, Color::Red) => acc.red = acc.red.max(num),
                        };
                        acc
                    })
                });
            tracker.blue * tracker.green * tracker.red
        })
        .sum()
}

fn is_possible(round: &(usize, Color)) -> bool {
    match *round {
        (n, Color::Blue) => n <= 14,
        (n, Color::Green) => n <= 13,
        (n, Color::Red) => n <= 12,
    }
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (rest, (_, num, _)) = tuple((tag("Game"), ws(digit1), ws(char(':'))))(input)?;
    let (rest, (matches, _)) = many_till(parse_match, eof)(rest)?;
    let game = Game {
        id: num.parse::<usize>().unwrap(),
        matches,
    };
    Ok((rest, game))
}

fn parse_match(input: &str) -> IResult<&str, Vec<(usize, Color)>> {
    let parse_round = terminated(tuple((ws(digit1), ws(alpha1))), opt(char(',')));
    let (rest, colors) = terminated(many0(parse_round), opt(char(';')))(input)?;
    let r#match: Vec<(usize, Color)> = colors
        .iter()
        .map(|(num, color)| {
            (
                num.parse::<usize>().unwrap(),
                Color::from_str(color).unwrap(),
            )
        })
        .collect();
    Ok((rest, r#match))
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example1() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let result = part_one(input);
        assert_eq!(result, 8)
    }

    #[test]
    fn part_one_test() {
        let input = include_str!("../resources/day2_part1");
        let result = part_one(input);
        assert_eq!(result, 2369)
    }

    #[test]
    fn part_two_example1() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let result = part_two(input);
        assert_eq!(result, 2286);
    }

    #[test]
    fn part_two_test() {
        let input = include_str!("../resources/day2_part1");
        let result = part_two(input);
        assert_eq!(result, 66363);
    }
}
