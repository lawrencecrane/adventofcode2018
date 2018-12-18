extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::fmt::Display;
use std::fmt;
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let mut f = File::open("data/input").unwrap();

    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let parsed = parse_square(&buffer);
    let resource_value = resource_value_after_n_ticks(parsed, 10);
    println!("Resource values after 10s: {}", resource_value);

    let parsed = parse_square(&buffer);
    let resource_value = resource_value_after_n_ticks(parsed, 1000000000);
    println!("Resource values after 1000000000s: {}", resource_value);
}

fn resource_value_after_n_ticks(area: Vec<Vec<Acre>>, n: usize) -> usize {
    let nrow = area.len();
    let ncol = area[0].len();

    let mut ntick = 0;
    let mut mutable_area = area;
    let mut resource_values = HashSet::new();
    let mut repeating_seq: Vec<usize> = Vec::new();
    let mut last_insert_tick = 0;

    let seq = 'clock: loop {
        // graph_area(&mutable_area);
        let value = resource_value(&mutable_area);

        match resource_values.contains(&value) {
            true => {
                match last_insert_tick + 1 == ntick {
                    true => {
                        match repeating_seq[0] {
                            // we found the repeating sequence:
                            val if val == value => {
                                let len = repeating_seq.len();
                                break 'clock Some((repeating_seq,
                                                   1 + last_insert_tick - len))
                            },
                            _ => {
                                // continue the repeating sequence:
                                repeating_seq.push(value);
                                last_insert_tick = ntick;
                            }
                        }
                    },
                    false => {
                        // start a new sequence:
                        repeating_seq.clear();
                        repeating_seq.push(value);
                        last_insert_tick = ntick;
                    }
                }
            },
            false => { resource_values.insert(value); }
        }

        mutable_area = tick(&mutable_area, nrow, ncol);
        ntick += 1;

        if ntick == n { break None }
    };

    match seq {
        Some((s, start_tick)) => {
            s[(n - start_tick) % s.len()]
        },
        None => resource_value(&mutable_area)
    }
}

fn resource_value(area: &Vec<Vec<Acre>>) -> usize {
    let (ntree, nyard) = area.iter()
        .flat_map(|acre| acre)
        .filter(|acre| acre != &&Acre::OPEN)
        .fold((0, 0), |(ntree, nyard), acre| {
            match acre {
                Acre::TREE => (ntree + 1, nyard),
                _ => (ntree, nyard + 1)
            }
        });

    ntree * nyard
}

fn graph_area(area: &Vec<Vec<Acre>>) {
    for row in area {
        for acre in row {
            print!("{}", acre);
        }
        println!("");
    }
    println!("");
}

fn tick(area: &Vec<Vec<Acre>>, nrow: usize, ncol: usize) -> Vec<Vec<Acre>> {
    // fst bruteforce way...
    (0..nrow).cartesian_product(0..ncol)
        .fold(vec![vec![Acre::OPEN; ncol]; nrow], |mut out, (i, j)| {
            let (ntree, nyard) = adjacent_idxs(i, j, nrow as isize, ncol as isize)
                .fold((0, 0), |(ntree, nyard), (x, y)| {
                    match area[x][y] {
                        Acre::TREE => (ntree + 1, nyard),
                        Acre::LUMBERYARD => (ntree, nyard + 1),
                        Acre::OPEN => (ntree, nyard)
                    }
                });
            out[i][j] = area[i][j].transform(ntree, nyard);
            out
        })
}

fn adjacent_idxs(i: usize, j: usize, nrow: isize, ncol: isize)
                 -> impl Iterator<Item = (usize, usize)> {
    let i = i as isize;
    let j = j as isize;
    (i-1..i+2).cartesian_product(j-1..j+2)
        .filter(move |(x, y)|
                x >= &0 && x < &nrow &&
                y >= &0 && y < &ncol &&
                !(x == &i && y == &j))
        .map(|(x, y)| (x as usize, y as usize))
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Acre {
    OPEN,
    TREE,
    LUMBERYARD
}

impl Acre {
    fn transform(&self, ntree_adj: usize, nyard_adj: usize) -> Acre {
        match self {
            Acre::OPEN => {
                match ntree_adj >= 3 {
                    true => Acre::TREE,
                    false => Acre::OPEN
                }
            },
            Acre::TREE => {
                match nyard_adj >= 3 {
                    true => Acre::LUMBERYARD,
                    false => Acre::TREE
                }
            },
            Acre::LUMBERYARD => {
                match ntree_adj >= 1 && nyard_adj >= 1 {
                    true => Acre::LUMBERYARD,
                    false => Acre::OPEN
                }
            }
        }
    }
}

impl Display for Acre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Acre::OPEN => write!(f, "{}", '.'),
            Acre::TREE => write!(f, "{}", '|'),
            Acre::LUMBERYARD => write!(f, "{}", '#')
        }
    }
}

fn parse_square(raw: &str) -> Vec<Vec<Acre>> {
    raw.lines()
        .fold(Vec::new(), |mut matrix, row| {
            let ij = row.chars()
                .fold(Vec::new(), |mut cols, col| {
                    cols.push(match col {
                        '#' => Acre::LUMBERYARD,
                        '|' => Acre::TREE,
                        _ => Acre::OPEN
                    });
                    cols
                });
            matrix.push(ij);
            matrix
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_value() {
        let raw = ".#.#...|#.\n\
                   .....#|##|\n\
                   .|..|...#.\n\
                   ..|#.....#\n\
                   #.#|||#|#|\n\
                   ...#.||...\n\
                   .|....|...\n\
                   ||...#|.#|\n\
                   |.||||..|.\n\
                   ...#.|..|.";

        let parsed = parse_square(raw);
        let resource_value = resource_value_after_n_ticks(parsed, 10);

        assert_eq!(resource_value, 1147);
    }
}
