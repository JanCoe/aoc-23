use std::collections::HashMap;

fn main() {
    let data = include_str!("../../../input/day-08.txt");
    let directions = get_directions(data);
    let nodes = get_nodes(data);
    let steps = follow_directions("AAA", "ZZZ", directions, &nodes);
    println!("steps: {steps}");
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn get_directions(data: &str) -> &str {
    data.lines().next().expect("first line has directions")
}

fn get_nodes(data: &str) -> HashMap<String, Node> {
    data.lines()
        .skip(2)
        .map(|line| {
            let mut chars = line.chars();
            (
                chars.by_ref().take(3).collect::<String>(),
                Node {
                    left: chars.by_ref().skip(4).take(3).collect::<String>(),
                    right: chars.by_ref().skip(2).take(3).collect::<String>(),
                },
            )
        })
        .collect()
}

fn follow_directions(
    start: &str,
    end: &str,
    directions: &str,
    nodes: &HashMap<String, Node>,
) -> usize {
    let mut current = start;

    for (ctr, direction) in directions.chars().cycle().enumerate() {
        if current == end {
            return ctr;
        }
        match direction {
            'L' => current = &nodes.get(current).unwrap().left,
            'R' => current = &nodes.get(current).unwrap().right,
            _ => panic!("invalid direction"),
        }
    }
    panic!("impossible to get here because above loop is infinite");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_get_directions() {
        assert_eq!(get_directions(TEST_DATA), "LLR");
    }

    #[test]
    fn test_get_nodes() {
        let nodes = get_nodes(TEST_DATA);
        assert_eq!(nodes.get("AAA").unwrap().left, "BBB");
        assert_eq!(nodes.get("AAA").unwrap().right, "BBB");
        assert_eq!(nodes.get("BBB").unwrap().left, "AAA");
        assert_eq!(nodes.get("BBB").unwrap().right, "ZZZ");
        assert_eq!(nodes.get("ZZZ").unwrap().left, "ZZZ");
        assert_eq!(nodes.get("ZZZ").unwrap().right, "ZZZ");
    }

    #[test]
    fn test_follow_directions() {
        let nodes = get_nodes(TEST_DATA);
        let directions = get_directions(TEST_DATA);
        let steps = follow_directions("AAA", "ZZZ", directions, &nodes);
        assert_eq!(6, steps);
    }
}
