use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Obstacle,
    Guard,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

pub fn day6_part1(input: &str) -> usize {
    let (mut x, mut y) = (0, 0);
    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, chr)| match chr {
                    '.' => Tile::Open,
                    '#' => Tile::Obstacle,
                    '^' => {
                        (x, y) = (row as isize, col as isize);
                        Tile::Guard
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    move_to_end(&tiles, Position { x, y }, Direction::Up)
}

fn move_to_end(tiles: &[Vec<Tile>], mut position: Position, mut direction: Direction) -> usize {
    let mut visited = HashSet::<Position>::new();
    visited.insert(position);
    while !(position.x - 1).is_negative()
        && !(position.y - 1).is_negative()
        && ((position.x + 1) as usize) < tiles.len()
        && ((position.y + 1) as usize) < tiles[0].len()
    {
        (direction, position) = tick(tiles, direction, position, &mut visited);
    }
    visited.len()
}

fn tick(
    tiles: &[Vec<Tile>],
    direction: Direction,
    position: Position,
    visited: &mut HashSet<Position>,
) -> (Direction, Position) {
    let next_position @ Position { x, y } = move_direction(position, direction);
    let tile = tiles[x as usize][y as usize];
    if tile == Tile::Obstacle {
        (rotate_90_degrees(direction), position)
    } else {
        visited.insert(next_position);
        (direction, next_position)
    }
}

fn rotate_90_degrees(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn move_direction(Position { x, y }: Position, direction: Direction) -> Position {
    match direction {
        Direction::Up => Position { x: x - 1, y },
        Direction::Right => Position { x, y: y + 1 },
        Direction::Down => Position { x: x + 1, y },
        Direction::Left => Position { x, y: y - 1 },
    }
}

#[cfg(test)]
mod test {
    use super::day6_part1;

    #[test]
    fn example_part1() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let result = day6_part1(input);
        assert_eq!(result, 41)
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("../resources/day6_part1");
        let result = day6_part1(input);
        assert_eq!(result, 5551)
    }
}
