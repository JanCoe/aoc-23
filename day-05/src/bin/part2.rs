#[derive(Debug)]
struct Map {
    from: u64,
    to: u64,
    delta: i64,
}

impl Map {
    fn new(data: Vec<u64>) -> Self {
        Self {
            from: data[1],
            to: data[1] + data[2] - 1,
            delta: data[0] as i64 - data[1] as i64,
        }
    }
}

#[derive(Debug)]
struct SeedRange {
    from: u64,
    to: u64,
}

fn parse_maps(data: &str) -> Vec<Map> {
    data.lines()
        .skip(1)
        .map(|line| {
            line.split(" ")
                .filter_map(|item| item.parse::<u64>().ok())
                .collect::<Vec<u64>>()
        })
        .map(|mapping| Map::new(mapping))
        .collect()
}

fn main() {
    let data: Vec<_> = include_str!("../../../input/day-05.txt")
        .split("\n\n")
        .collect();

    let seed_ranges: Vec<_> = data[0]
        .split(" ")
        .skip(1)
        .filter_map(|x| x.parse::<u64>().ok())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|x| SeedRange {
            from: x[0],
            to: x[0] + x[1] - 1,
        })
        .collect();

    let mut maps: Vec<Vec<Map>> = vec![parse_maps(data[1])];
    for i in 2..=7 {
        maps.push(parse_maps(data[i]));
    }

    let mut lowest_location: Option<u64> = None;

    for seed_range in seed_ranges {
        for seed in seed_range.from..=seed_range.to {
            let mut mapped = seed;

            for map_type in &maps {
                for map in map_type {
                    if mapped >= map.from && mapped <= map.to {
                        mapped = (mapped as i64 + map.delta) as u64;
                        break;
                    }
                }
            }

            match lowest_location {
                None => lowest_location = Some(mapped),
                Some(lowest) => lowest_location = Some(lowest.min(mapped)),
            }
        }
    }
    println!("lowest_location: {:?}", lowest_location.unwrap());
}
