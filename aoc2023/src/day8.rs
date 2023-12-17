use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::Lines,
};

pub fn part_one(input: &str) -> usize {
    let mut lines = input.lines();
    let steps = lines.next().unwrap();
    let graph = build_lookup(lines);
    let directions = steps.chars().collect::<VecDeque<char>>();
    bfs_with_map("AAA", graph, directions)
}

fn bfs_with_map(
    root: &str,
    graph: HashMap<String, (String, String)>,
    mut directions: VecDeque<char>,
) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((root, 0));
    seen.insert(root);
    while !queue.is_empty() {
        let (root, steps) = queue.pop_front().unwrap();
        if root == "ZZZ" {
            return steps;
        }
        let (left, right) = graph.get(root).unwrap();
        let r#move = directions.pop_front().unwrap();
        directions.push_back(r#move);
        match r#move {
            'L' => queue.push_back((left, steps + 1)),
            'R' => queue.push_back((right, steps + 1)),
            _ => unreachable!(),
        }
    }
    0
}

fn build_lookup(lines: Lines) -> HashMap<String, (String, String)> {
    lines.skip(1).fold(HashMap::new(), |mut acc, line| {
        let matches: Vec<&str> = line.matches(char::is_alphabetic).collect();
        let mut paths = matches
            .chunks_exact(3)
            .map(|chunk| {
                chunk.iter().fold(String::new(), |mut acc, s| {
                    acc.push_str(s);
                    acc
                })
            })
            .collect::<Vec<_>>();
        acc.insert(
            paths.remove(0).to_owned(),
            (paths.remove(0), paths.remove(0)),
        );
        acc
    })
}

pub fn part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example1() {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
        let result = part_one(input);
        assert_eq!(result, 6)
    }

    #[test]
    fn part_one_test() {
        let input = include_str!("../resources/day8_part1");
        let result = part_one(input);
        assert_eq!(result, 0)
    }

    #[test]
    fn part_two_example1() {
        let input = r#""#;
        let result = part_one(input);
        assert_eq!(result, 0)
    }

    #[test]
    fn part_two_test() {
        let input = r#""#;
        let result = part_one(input);
        assert_eq!(result, 0)
    }
}
