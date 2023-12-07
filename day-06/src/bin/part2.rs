#[derive(Debug)]
struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn distance(&self, hold_time: usize) -> usize {
        hold_time * (self.time - hold_time)
    }

    fn win(&self, distance: usize) -> bool {
        distance > self.record
    }

    fn ways_to_win(&self) -> usize {
        (1..self.time)
            .filter(|&hold_time| self.win(self.distance(hold_time)))
            .count()
    }
}

fn remove_char(s: &str, c: char) -> String {
    s.chars().filter(|&x| x != c).collect()
}

fn grab_numbers(s: &str) -> usize {
    let line: String = s.split(':').skip(1).collect();
    remove_char(&line, ' ').parse().unwrap()
}

fn parse_data(data: &str) -> Race {
    let mut lines = data.lines();
    Race {
        time: grab_numbers(lines.next().expect("first line exists")),
        record: grab_numbers(lines.next().expect("second line exists")),
    }
}

fn main() {
    let data = include_str!("../../../input/day-06.txt");
    let race = parse_data(data);
    let answer = race.ways_to_win();
    println!("Answer: {}", answer);
}
