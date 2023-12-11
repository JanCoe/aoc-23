use itertools::Itertools;
use std::collections::{BTreeMap, HashSet};

const EXPAND: usize = 999999; //1 for part1, 999999 for part2

fn find_galaxies(data: &str) -> BTreeMap<usize, (usize, usize)> {
    let mut galaxies = BTreeMap::new();

    let mut ctr = 1;
    for (row_ctr, row) in data.lines().enumerate() {
        for (col_ctr, char) in row.chars().enumerate() {
            if char == '#' {
                galaxies.insert(ctr, (row_ctr, col_ctr));
                ctr += 1;
            }
        }
    }
    galaxies
}

fn expand_counters(blanks: &Vec<usize>, num: usize) -> Vec<usize> {
    let mut expand_vec = vec![0; num];
    let mut expand_ctr = 0;
    for i in 0..num {
        if blanks.contains(&i) {
            expand_ctr += EXPAND;
        };
        expand_vec[i] = expand_ctr;
    }
    expand_vec
}

fn find_blanks(
    galaxies: &BTreeMap<usize, (usize, usize)>,
    num_rows: usize,
    num_cols: usize,
) -> (Vec<usize>, Vec<usize>) {
    let rows: HashSet<_> = galaxies.iter().map(|(_, (row, _))| *row).collect();
    let cols: HashSet<_> = galaxies.iter().map(|(_, (_, col))| *col).collect();

    let all_rows: HashSet<_> = (0..num_rows).collect();
    let all_cols: HashSet<_> = (0..num_cols).collect();

    let mut blank_rows: Vec<_> = all_rows.difference(&rows).copied().collect();
    let mut blank_cols: Vec<_> = all_cols.difference(&cols).copied().collect();

    blank_rows.sort();
    blank_cols.sort();

    (blank_rows, blank_cols)
}

fn main() {
    let data = include_str!("../../../input/day-11.txt");

    let num_rows = data.lines().count();
    let num_cols = data.lines().next().unwrap().chars().count();

    let mut galaxies = find_galaxies(data);

    let (blank_rows, blank_cols) = find_blanks(&galaxies, num_rows, num_cols);

    let row_expand_vec = expand_counters(&blank_rows, num_rows);
    let col_expand_vec = expand_counters(&blank_cols, num_cols);

    for (row, col) in galaxies.values_mut() {
        *row += row_expand_vec[*row];
        *col += col_expand_vec[*col];
    }

    let mut total_distance = 0;
    galaxies.keys().combinations(2).for_each(|pair| {
        let galaxy1 = galaxies.get(&pair[0]).unwrap();
        let galaxy2 = galaxies.get(&pair[1]).unwrap();
        let distance = (galaxy1.0 as isize - galaxy2.0 as isize).abs()
            + (galaxy1.1 as isize - galaxy2.1 as isize).abs();
        total_distance += distance;
    });
    println!("total distance: {}", total_distance);
}
