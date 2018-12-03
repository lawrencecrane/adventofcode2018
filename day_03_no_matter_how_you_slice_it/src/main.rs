use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let mut f = File::open("data/day_03_input")
        .expect("File not found");

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Something went wrong reading the file");

    println!("Overlapping square inches: {}", draw_canvas(&buffer));
    println!("Intact id: {}", get_intact_id(&buffer));
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize
}

fn get_intact_id(buffer: &String) -> usize {
    // this could be created by scanning through the input and finding the smallest and biggest values:
    let canvas: Vec<Vec<Option<usize>>> = vec![vec![None; 5000]; 5000];

    let mut intact_ids = buffer.lines()
        .map(parse_claim)
        .fold((canvas, HashSet::new()), |acc, x| draw_square_ids(acc.0, acc.1, x) );

    let intact_ids: Vec<usize> = intact_ids.1.drain().collect();
    assert!(intact_ids.len() == 1);

    intact_ids[0]
}

fn draw_square_ids(mut canvas: Vec<Vec<Option<usize>>>, mut intact_ids: HashSet<usize>, square: Claim) -> (Vec<Vec<Option<usize>>>, HashSet<usize>) {
    intact_ids.insert(square.id);

    for i in square.top..square.top + square.height {
        for j in square.left..square.left + square.width {
            // if first, draw id:
            if canvas[i][j] == None { canvas[i][j] = Some(square.id); }
            // else mask it:
            else {
                intact_ids.remove(&canvas[i][j].expect(""));
                intact_ids.remove(&square.id);
                canvas[i][j] = Some(0);
            }
        }
    }

    (canvas, intact_ids)
}

fn draw_canvas(buffer: &String) -> usize {
    // this could be created by scanning through the input and finding the smallest and biggest values:
    let canvas = vec![vec![0; 5000]; 5000];
    let overlapping_units = buffer.lines()
        .map(parse_claim)
        .fold(canvas, draw_square)
        .iter()
        .flatten()
        .filter(|&&x| x > 1)
        .count();

    overlapping_units
}

fn draw_square(mut canvas: Vec<Vec<usize>>, square: Claim) -> Vec<Vec<usize>> {
    for i in square.top..square.top + square.height {
        for j in square.left..square.left + square.width {
            canvas[i][j] = canvas[i][j] + 1;
        }
    }

    canvas
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
    fn test_draw_canvas() {
        assert_eq!(draw_canvas(&String::from("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2")), 4);
    }

    #[test]
    fn test_get_intact_id() {
        assert_eq!(get_intact_id(&String::from("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2")), 3);
    }
}
