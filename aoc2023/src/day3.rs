use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{digit1, line_ending},
    combinator::eof,
    multi::{many1, many_till},
    IResult,
};
use nom_locate::{position, LocatedSpan};

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct BoundingBox {
    start: Point,
    end: Point,
    value: usize,
}

impl BoundingBox {
    fn contains(&self, point: &Point) -> bool {
        point.x >= self.start.x
            && point.x <= self.end.x
            && point.y >= self.start.y
            && point.y <= self.end.y
    }
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Number,
    Dot,
    Symbol,
}

#[derive(Debug)]
struct Token<'a> {
    position: Span<'a>,
    token: &'a str,
    token_type: TokenType,
}

impl<'a> Token<'a> {
    fn new(position: Span<'a>, token: &'a str, token_type: TokenType) -> Self {
        Self {
            position,
            token,
            token_type,
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let span = Span::new(input);
    let (_, tokens) = parse(span).unwrap();
    let tokens = tokens
        .into_iter()
        .flat_map(|(tokens, _)| tokens)
        .collect::<Vec<Token<'_>>>();
    let boxes = build_bounding_boxes(&tokens);
    let symbols = build_symbol_points(&tokens);
    boxes
        .into_iter()
        .flat_map(|r#box| {
            symbols
                .iter()
                .any(|symbol| r#box.contains(symbol))
                .then_some(r#box.value)
                .or(Some(0))
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let span = Span::new(input);
    let (_, tokens) = parse(span).unwrap();
    let tokens = tokens
        .into_iter()
        .flat_map(|(tokens, _)| tokens)
        .collect::<Vec<Token<'_>>>();
    let boxes = build_bounding_boxes(&tokens);
    let symbols = build_symbol_points(&tokens);
    symbols
        .into_iter()
        .map(|symbol| compute_product_for_symbol(symbol, &boxes))
        .sum()
}

fn compute_product_for_symbol(symbol: Point, boxes: &[BoundingBox]) -> usize {
    boxes
        .iter()
        .try_fold((0, 1), |acc @ (count, intermediate), r#box| {
            if count > 2 {
                return None;
            }
            if r#box.contains(&symbol) {
                Some((count + 1, intermediate * r#box.value))
            } else {
                Some(acc)
            }
        })
        .filter(|(count, _)| *count == 2)
        .unwrap_or((0, 0))
        .1
}

fn filter_map_tokens<T>(
    tokens: &[Token<'_>],
    filter_on: TokenType,
    f: impl FnMut(&Token) -> T,
) -> Vec<T> {
    tokens
        .iter()
        .filter(|token| token.token_type == filter_on)
        .map(f)
        .collect::<Vec<T>>()
}

fn build_bounding_boxes(tokens: &[Token<'_>]) -> Vec<BoundingBox> {
    let f = |token: &Token| {
        let line = token.position.location_line() as usize;
        let column = token.position.get_column();
        let length = token.token.len();
        let number = token.token.parse::<usize>().unwrap();
        BoundingBox {
            start: Point::new(column.saturating_sub(length + 2), line.saturating_sub(2)),
            end: Point::new(column - 1, line),
            value: number,
        }
    };
    filter_map_tokens(tokens, TokenType::Number, f)
}

fn build_symbol_points(tokens: &[Token<'_>]) -> Vec<Point> {
    let f = |token: &Token| {
        let line = token.position.location_line() as usize;
        let column = token.position.get_column();
        Point::new(column - 2, line - 1)
    };
    filter_map_tokens(tokens, TokenType::Symbol, f)
}

fn parse(span: Span) -> IResult<Span, Vec<(Vec<Token<'_>>, Span<'_>)>> {
    let (rest, (tokens, _)) = many_till(
        many_till(
            alt((parse_number, parse_dot, parse_symbol)),
            alt((line_ending, eof)),
        ),
        eof,
    )(span)?;
    Ok((rest, tokens))
}

fn parse_number(span: Span) -> IResult<Span, Token> {
    let (rest, number) = digit1(span)?;
    let (rest, position) = position(rest)?;
    Ok((
        rest,
        Token::new(position, number.fragment(), TokenType::Number),
    ))
}

fn parse_dot(input: Span) -> IResult<Span, Token> {
    let (rest, _) = many1(tag("."))(input)?;
    let (rest, position) = position(rest)?;
    Ok((rest, Token::new(position, ".", TokenType::Dot)))
}

fn parse_symbol(span: Span) -> IResult<Span, Token> {
    let (rest, symbol) = is_a("#*+$&-%/=@")(span)?;
    let (rest, position) = position(rest)?;
    Ok((
        rest,
        Token::new(position, symbol.fragment(), TokenType::Symbol),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let result = part_one(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("../resources/day3_part1");
        let result = part_one(input);
        assert_eq!(result, 533775);
    }

    #[test]
    fn part_two_example1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let result = part_two(input);
        assert_eq!(result, 467835);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../resources/day3_part1");
        let result = part_two(input);
        assert_eq!(result, 78236071);
    }
}
