use std::collections::{HashMap, HashSet};

const TEST_DATA: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

fn main() {
    let data = include_str!("../../../input/day-08.txt");
    // let data = TEST_DATA;
    let directions = get_directions(data);
    let nodes = get_nodes(data);
    let get_locations = get_locations(&nodes);
    let locations_ending_with_a = locations_end_with(&get_locations, "A");

    let steps = follow_directions(locations_ending_with_a, "Z", directions, &nodes);
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

fn get_locations(nodes: &HashMap<String, Node>) -> HashSet<String> {
    let mut locations = HashSet::new();
    for (location, _) in nodes {
        locations.insert(location.clone());
    }
    locations
}

fn locations_end_with(locations: &HashSet<String>, end: &str) -> Vec<String> {
    locations
        .iter()
        .filter(|location| location.ends_with(end))
        .map(|location| location.clone())
        .collect()
}

fn check_locations_end_with(locations: &Vec<String>, end: &str) -> bool {
    locations.len()
        == locations
            .iter()
            .filter(|location| location.ends_with(end))
            .count()
}

fn follow_directions(
    start: Vec<String>,
    end: &str,
    directions: &str,
    nodes: &HashMap<String, Node>,
) -> usize {
    let mut current = start;

    for (ctr, direction) in directions.chars().cycle().enumerate() {
        if check_locations_end_with(&current, end) {
            return ctr;
        }
        if ctr % 100000 == 0 {
            println!("ctr {}: {:?}", ctr, current);
        }
        match direction {
            'L' => {
                current = current
                    .iter()
                    .map(|loc| nodes.get(loc).unwrap().left.clone())
                    .collect()
            }
            'R' => {
                current = current
                    .iter()
                    .map(|loc| nodes.get(loc).unwrap().right.clone())
                    .collect()
            }
            _ => panic!("invalid direction"),
        }
        // println!("current: {:?}", current);
    }
    panic!("impossible to get here because above loop is infinite");
}

#[cfg(test)]
mod tests {
    use super::{follow_directions, get_directions, get_locations, get_nodes, locations_end_with};

    const TEST_DATA: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_get_directions() {
        assert_eq!(get_directions(TEST_DATA), "LR");
    }

    #[test]
    fn test_get_nodes() {
        let nodes = get_nodes(TEST_DATA);
        assert_eq!(nodes.get("11A").unwrap().left, "11B");
        assert_eq!(nodes.get("11A").unwrap().right, "XXX");
        assert_eq!(nodes.get("22C").unwrap().left, "22Z");
        assert_eq!(nodes.get("XXX").unwrap().right, "XXX");
    }

    #[test]
    fn test_get_locations() {
        let nodes = get_nodes(TEST_DATA);
        let locations = get_locations(&nodes);
        assert_eq!(locations.len(), 8);
        assert!(locations.contains("11A"));
    }

    #[test]
    fn test_location_ends_with() {
        let nodes = get_nodes(TEST_DATA);
        let locations = get_locations(&nodes);
        let locations_ending_with_z = locations_end_with(&locations, "Z");
        assert_eq!(locations_ending_with_z.len(), 2);
        assert!(locations_ending_with_z.contains(&"11Z".to_string()));
    }

    #[test]
    fn test_follow_directions() {
        let directions = get_directions(TEST_DATA);
        let nodes = get_nodes(TEST_DATA);
        let locations = get_locations(&nodes);
        let locations_ending_with_a = locations_end_with(&locations, "A");
        let steps = follow_directions(locations_ending_with_a, "Z", directions, &nodes);
        assert_eq!(6, steps);
    }
}
