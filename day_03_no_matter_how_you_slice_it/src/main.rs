extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let mut f = File::open("data/day_03_input")
        .expect("File not found");

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Something went wrong reading the file");

    let answer = populate_canvas(&buffer);

    println!("Overlapping square inches: {}", answer.0);
    println!("Intact id: {}", answer.1);
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize
}

fn populate_canvas(buffer: &str) -> (usize, usize) {
    let (canvas, mut intact_id) = buffer.lines()
        .map(parse_claim)
        .fold((HashMap::new(), HashSet::new()),
              |acc, sqr| add_square(acc.0, acc.1, sqr));

    let intact_id: Vec<usize> = intact_id.drain().collect();
    assert!(intact_id.len() == 1);

    let overlapping_units = canvas.values()
        .filter(|&&x| x == 0)
        .count();

    (overlapping_units, intact_id[0])
}

fn add_square(canvas: HashMap<(usize, usize), usize>,
              mut intact_ids: HashSet<usize>,
              square: Claim) ->(HashMap<(usize, usize), usize>, HashSet<usize>) {

    intact_ids.insert(square.id);

    (square.top..square.top + square.height)
        .cartesian_product(square.left..square.left + square.width)
        .fold((canvas, intact_ids), |(mut can, mut ii), point| {
            can.entry(point)
            // if the entry already exists:
            .and_modify(|old_id| {
              ii.remove(&*old_id);
              ii.remove(&square.id);
              *old_id = 0;
            })
            // if the entry doesn't exist:
            .or_insert(square.id);

            (can, ii)
        })
}

fn parse_claim(row: &str) -> Claim {
    let values: Vec<&str> =  row.split(' ').collect();
    assert!(values.len() == 4);
    assert!(values[1] == "@");

    let id: usize = values[0].replace("#", "").parse().expect("Not a number");

    let position: Vec<&str> = values[2].split(",").collect();
    let (left, top): (usize, usize) = (position[0].parse().expect("Not a number"),
                       position[1].replace(":", "").parse().expect("Not a number"));

    let size: Vec<&str> = values[3].split("x").collect();
    let (width, height): (usize, usize) = (size[0].parse().expect("Not a number"),
                                       size[1].parse().expect("Not a number"));

    Claim {
        id: id,
        left: left,
        top: top,
        width: width,
        height: height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_claim() {
        assert_eq!(parse_claim("#1 @ 393,863: 11x29"),
                   Claim {
                       id: 1,
                       left: 393,
                       top: 863,
                       width: 11,
                       height: 29
                   }
        );
    }

    #[test]
    fn test_populate_canvas() {
        let answer = populate_canvas(&String::from("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
        assert_eq!(answer.0, 4);
        assert_eq!(answer.1, 3);
    }
}
