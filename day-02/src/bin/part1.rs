fn main() {
    let content = include_str!("../../../input/day-02.txt");
    let total: u32 = content.lines().map(|line| id_of_valid_game(line)).sum();
    println!("Total: {:?}", total);
}

fn id_of_valid_game(line: &str) -> u32 {
    let blue = 14;
    let red = 12;
    let green = 13;

    let mut valid = true;

    let mut line = line.split(": ");
    let game_id: u32 = line
        .next()
        .unwrap()
        .split(" ")
        .last()
        .expect("a string")
        .parse()
        .unwrap();
    let boxes = line.last().unwrap();

    let turns: Vec<_> = boxes.split("; ").collect();

    for turn in &turns {
        let selections: Vec<_> = turn.split(", ").collect();

        for selection in &selections {
            let go: Vec<_> = selection.split(" ").collect();
            let number: u32 = go[0].parse().unwrap();
            match go[1] {
                "blue" => {
                    if number > blue {
                        valid = false
                    }
                }
                "red" => {
                    if number > red {
                        valid = false
                    }
                }
                "green" => {
                    if number > green {
                        valid = false
                    }
                }
                _ => panic!("wrong colour"),
            }
        }
    }

    if valid {
        game_id
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::id_of_valid_game;

    #[test]
    fn test_valid_game() {
        let content = include_str!("../../../input/day-02_test.txt");
        let total: u32 = content.lines().map(|line| id_of_valid_game(line)).sum();
        assert_eq!(total, 8);
    }
}
