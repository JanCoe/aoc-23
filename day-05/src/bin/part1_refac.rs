use std::collections::{HashSet, BTreeMap};

#[derive(Debug, Clone)]
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
    let data: Vec<_> = include_str!("../../../input/day-05_test.txt")
        .split("\n\n")
        .collect();

    let seeds: HashSet<u64> = data[0]
        .split(" ")
        .skip(1)
        .filter_map(|x| x.parse().ok())
        .collect();

    let mut maps: Vec<Vec<Map>> = vec![parse_maps(data[1])];
    for i in 2..=7 {
        maps.push(parse_maps(data[i]));
    }

    let mut btree = BTreeMap::<u64, i64>::new();
    //btree.insert(0, 0);

    for map_type in maps {
        for map in map_type {
            println!("processing map: {:?}", map);

            match btree.get_mut(&map.from) {
                Some(delta) => *delta += map.delta,
                None => {
                    btree.insert(map.from, map.delta);
                }
            }
            println!("btree first: {:?}", btree);

//            let mut previous_delta = btree.get(&previous_key).unwrap().clone();
            let mut previous_delta = 0;

            for (key, delta) in btree.iter_mut() {
                if key > &map.from && key <= &map.to {
                    previous_delta = *delta;
                    *delta += map.delta;
                };
            }
            println!("btree second: {:?}, previous delta: {:?}", btree, previous_delta);
            match btree.get_mut(&(&map.to+1)) {
                None => btree.insert(map.to+1, previous_delta),
                _ => None,
            };
            println!("btree third: {:?} \n", btree);
        }
    }
}
