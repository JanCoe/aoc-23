#[derive(Debug)]
struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    fn distance(&self, hold_time: usize) -> usize {
        hold_time * (self.time - hold_time)
    }

    fn win(&self, distance: usize) -> bool {
        distance > self.record_distance
    }

    fn ways_to_win(&self) -> usize {
        (1..self.time)
            .filter(|&hold_time| self.win(self.distance(hold_time)))
            .count()
    }
}

fn parse_data(data: &str) -> Vec<Race> {
    let mut lines = data.lines();
    lines
        .next()
        .expect("first line exists")
        .split(' ')
        .skip(1)
        .filter_map(|x| x.parse::<usize>().ok())
        .zip(
            lines
                .next()
                .expect("second line exists")
                .split(' ')
                .skip(1)
                .filter_map(|x| x.parse::<usize>().ok()),
        )
        .map(|(t, d)| Race {
            time: t,
            record_distance: d,
        })
        .collect()
}

fn main() {
    let data = include_str!("../../../input/day-06.txt");
    let races = parse_data(data);

    let answer: usize = races.iter().map(|race| race.ways_to_win()).product();
    println!("Answer: {}", answer);
}
