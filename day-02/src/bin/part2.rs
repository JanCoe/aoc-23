fn main() {
    let content = include_str!("../../../input/day-02.txt");
    let mut total = 0;

    for line in content.lines() {
        total += power_of_game(line);
    }
    println!("Total: {:?}", total);
}

fn power_of_game(line: &str) -> u32 {
    let mut blue_cubes = 0;
    let mut red_cubes = 0;
    let mut green_cubes = 0;

    let line = line.split(": ");
    let boxes = line.last().unwrap();
    let turns = boxes.split("; ").collect::<Vec<&str>>();

    for turn in &turns {
        let selections = turn.split(", ").collect::<Vec<&str>>();

        for selection in &selections {
            let go = selection.split(" ").collect::<Vec<&str>>();

            let number = go[0].parse::<u32>().unwrap();
            match go[1] {
                "blue" => if number > blue_cubes {blue_cubes = number},
                "red" => if number > red_cubes {red_cubes = number},
                "green" => if number > green_cubes {green_cubes = number},
                _ => panic!("wrong colour"),
            }
        }
    }
    blue_cubes * red_cubes * green_cubes
}


#[cfg(test)]
mod tests {
    use super::{power_of_game};

    #[test]
    fn test_power_of_game() {
        let content = include_str!("../../../input/day-02_test.txt");
        let mut total = 0;

        for line in content.lines() {
            total += power_of_game(line);
        }
        assert_eq!(total, 2286);
    }
}