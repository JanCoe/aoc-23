#[derive(Debug, Clone)]
struct Point(usize, usize);

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Grid<'a> {
    rows: Vec<&'a str>,
    start: Point,
    direction: Direction,
}

impl Grid<'_> {
    fn new(content: &str) -> Grid {
        let rows: Vec<_> = content.lines().collect();
        let start = find_start(content);
        let direction = get_start_direction(&rows, &start, Direction::East);
        Grid {
            rows,
            start,
            direction,
        }
    }
}

fn find_start(content: &str) -> Point {
    for (row, line) in content.lines().enumerate() {
        for (col, symbol) in line.chars().enumerate() {
            if symbol == 'S' {
                return Point(row, col);
            }
        }
    }
    panic!("No start found");
}

fn get_start_direction(rows: &Vec<&str>, start: &Point, guess: Direction) -> Direction {
    let mut direction = guess;

    loop {
        match direction {
            Direction::North => {
                if ['|', '7', 'F'].contains(&get(rows, &take_step(start, &Direction::North))) {
                    return direction;
                } else {
                    direction = Direction::East;
                }
            }
            Direction::East => {
                if ['-', '7', 'J'].contains(&get(rows, &take_step(start, &Direction::East))) {
                    return direction;
                } else {
                    direction = Direction::South;
                }
            }
            Direction::South => {
                if ['|', 'J', 'L'].contains(&get(rows, &take_step(start, &Direction::South))) {
                    return direction;
                } else {
                    direction = Direction::West;
                }
            }
            Direction::West => {
                if ['-', 'F', 'L'].contains(&get(rows, &take_step(start, &Direction::West))) {
                    return direction;
                } else {
                    direction = Direction::North;
                }
            }
        }
    }
}

fn get(rows: &Vec<&str>, point: &Point) -> char {
    rows[point.0].chars().nth(point.1).unwrap()
}

fn take_step(current: &Point, direction: &Direction) -> Point {
    match direction {
        Direction::North => Point(current.0 - 1, current.1),
        Direction::South => Point(current.0 + 1, current.1),
        Direction::East => Point(current.0, current.1 + 1),
        Direction::West => Point(current.0, current.1 - 1),
    }
}

fn next_direction(grid: &Grid, current: &Point, direction: Direction) -> Direction {
    let symbol = get(&grid.rows, &current);

    if direction == Direction::North {
        match symbol {
            '|' => return Direction::North,
            'F' => return Direction::East,
            '7' => return Direction::West,
            _ => panic!("invalid symbol"),
        }
    } else if direction == Direction::South {
        match symbol {
            '|' => return Direction::South,
            'J' => return Direction::West,
            'L' => return Direction::East,
            _ => panic!("invalid symbol"),
        }
    } else if direction == Direction::East {
        match symbol {
            '-' => return Direction::East,
            'J' => return Direction::North,
            '7' => return Direction::South,
            _ => panic!("invalid symbol"),
        }
    } else {
        match symbol {
            '-' => return Direction::West,
            'L' => return Direction::North,
            'F' => return Direction::South,
            _ => panic!("invalid symbol"),
        }
    }
}

fn main() {
    let content = include_str!("../../../input/day-10.txt");
    let grid = Grid::new(content);

    let mut current = grid.start.clone();
    let mut direction = grid.direction.clone();

    current = take_step(&current, &direction);
    let mut steps = 1;

    loop {
        direction = next_direction(&grid, &current, direction);
        current = take_step(&current, &direction);
        steps += 1;

        if current.0 == grid.start.0 && current.1 == grid.start.1 {
            println!("Furthest point: {}", steps / 2);
            return;
        }
    }
}
