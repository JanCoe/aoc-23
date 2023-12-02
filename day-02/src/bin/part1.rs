fn main() {
    let content = include_str!("../../../input/day-02.txt");
    let mut total = 0;

    for line in content.lines() {
        total += id_of_valid_game(line);
    }
    println!("Total: {:?}", total);
}

fn id_of_valid_game(line: &str) -> u32 {
    let total_blue_cubes = 14;
    let total_red_cubes = 12;
    let total_green_cubes = 13;

    let mut valid_game = true;

    let mut line = line.split(": ");
    let game_id = line.next().unwrap().split(" ").last().expect("a string").parse::<u32>().unwrap();
    let boxes = line.last().unwrap();

    let turns = boxes.split("; ").collect::<Vec<&str>>();

    for turn in &turns {
        let selections = turn.split(", ").collect::<Vec<&str>>();

        for selection in &selections {
            let go = selection.split(" ").collect::<Vec<&str>>();

            let number = go[0].parse::<u32>().unwrap();
            match go[1] {
                "blue" => if number > total_blue_cubes {valid_game = false},
                "red" => if number > total_red_cubes {valid_game = false},
                "green" => if number > total_green_cubes {valid_game = false},
                _ => panic!("wrong colour"),
            }
        }
    }

    if valid_game {
        game_id
    } else {
        0
    }
}


#[cfg(test)]
mod tests {
    use super::{id_of_valid_game};

    #[test]
    fn test_valid_game() {
        let content = include_str!("../../../input/day-02_test.txt");
        let mut total = 0;

        for line in content.lines() {
            total += id_of_valid_game(line);
        }
        assert_eq!(total, 8);
    }
}