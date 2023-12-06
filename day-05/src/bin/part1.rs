use std::collections::HashSet;
use std::iter::once;

#[derive(Debug)]
struct Map {
    from: u32,
    to: u32,
    delta: i32,
}

impl Map {
    fn new(data: Vec<u32>) -> Self {
        Self { from: data[1], to: data[1] + data[2] - 1, delta: data[0] as i32 - data[1] as i32 }
    }
}

fn main() {
    let data: Vec<_> = include_str!("../../../input/day-05_test.txt")
        .split("\n\n")
        .collect();

    println!("data: {:?}", data);

    let seeds: HashSet<u32> = data[0]
        .split(" ")
        .skip(1)
        .filter_map(|x| x.parse().ok())
        .collect();

    println!("seeds: {:?}", seeds);

    println!("data[1]: {:?}", data[1]);

    let seed_to_soil = data[1].lines().collect::<Vec<&str>>();

    println!("seed_to_soil: {:?}", seed_to_soil);

    let seed_to_soil: Vec<_> = seed_to_soil
        .iter()
        .skip(1)
        .map(|x| {
            x.split(" ")
                .filter_map(|y| y.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .collect();

    let seed_to_soil: Vec<_> = seed_to_soil
        .iter()
        .map(|x| Map::new(x.to_vec()))
        .collect();

    //             .filter_map(|line| line.parse().ok())
    //            .collect();
    //   }

    println!("seed_to_soil: {:?}", seed_to_soil);

    /*
    let seed_to_soil: HashSet<Map> = seed_to_soil
        .iter()
        .map(|line| {
            let mut line = line.split(" ");
            let destination = line.next().unwrap().parse::<u32>().unwrap();
            let source = line.next().unwrap().parse::<u32>().unwrap();
            let range = line.next().unwrap().parse::<u32>().unwrap();

            Map {
                from: source,
                to: source + range - 1,
                delta: destination - source,
            }
        })
        .collect();

     */
    /*
        .iter()
        .map(|(destination, source, range)| Map {
            from: source,
            to: source + range - 1,
            delta: destination - source,
        })
        .collect();

    println!("seed_to_soil: {:?}", seed_to_soil);
     */
    //    println!("seed_to_soil: {:?}", seed_to_soil);
    /*    .for_each(|line| {
        let mut line = line.lines();
        let seed = line.next().unwrap();
        let seeds: HashSet<_> = seed
            .split(" ")
            .skip(1)
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();

        println!("seeds: {:?}", seeds);
    });


    let line = data.lines().next().unwrap();
    let seeds: HashSet<_> = line
        .split(" ")
        .skip(1)
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    println!("seeds: {:?}", seeds);

     */
}
