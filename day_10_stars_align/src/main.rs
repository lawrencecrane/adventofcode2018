extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::cmp;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let mut f = File::open("data/day_10_input").unwrap();

    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    // const INPUT: &str = "position=< 9,  1> velocity=< 0,  2>\n\
    //                      position=< 7,  0> velocity=<-1,  0>\n\
    //                      position=< 3, -2> velocity=<-1,  1>\n\
    //                      position=< 6, 10> velocity=<-2, -1>\n\
    //                      position=< 2, -4> velocity=< 2,  2>\n\
    //                      position=<-6, 10> velocity=< 2, -2>\n\
    //                      position=< 1,  8> velocity=< 1, -1>\n\
    //                      position=< 1,  7> velocity=< 1,  0>\n\
    //                      position=<-3, 11> velocity=< 1, -2>\n\
    //                      position=< 7,  6> velocity=<-1, -1>\n\
    //                      position=<-2,  3> velocity=< 1,  0>\n\
    //                      position=<-4,  3> velocity=< 2,  0>\n\
    //                      position=<10, -3> velocity=<-1,  1>\n\
    //                      position=< 5, 11> velocity=< 1, -2>\n\
    //                      position=< 4,  7> velocity=< 0, -1>\n\
    //                      position=< 8, -2> velocity=< 0,  1>\n\
    //                      position=<15,  0> velocity=<-2,  0>\n\
    //                      position=< 1,  6> velocity=< 1,  0>\n\
    //                      position=< 8,  9> velocity=< 0, -1>\n\
    //                      position=< 3,  3> velocity=<-1,  1>\n\
    //                      position=< 0,  5> velocity=< 0, -1>\n\
    //                      position=<-2,  2> velocity=< 2,  0>\n\
    //                      position=< 5, -2> velocity=< 1,  2>\n\
    //                      position=< 1,  4> velocity=< 2,  1>\n\
    //                      position=<-2,  7> velocity=< 2, -2>\n\
    //                      position=< 3,  6> velocity=<-1, -1>\n\
    //                      position=< 5,  0> velocity=< 1,  0>\n\
    //                      position=<-6,  0> velocity=< 2,  0>\n\
    //                      position=< 5,  9> velocity=< 1, -2>\n\
    //                      position=<14,  7> velocity=<-2,  0>\n\
    //                      position=<-3,  6> velocity=< 2, -1>";

    // let mut vector_space: Vec<Vector> = String::from(INPUT).lines()
    //     .map(parse_line_to_tuple)
    //     .collect();

    let mut vector_space: Vec<Vector> = buffer.lines()
        .map(parse_line_to_tuple)
        .collect();


    let time = find_most_compact_space_time(vector_space.clone(), 100);

    println!("Time: {}", time);

    tick(&mut vector_space, time as isize);
    draw_space(&vector_space);
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Vector {
    coord: Point,
    i: isize,
    j: isize
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

fn find_most_compact_space_time(mut vector_space: Vec<Vector>, n: usize) -> usize {
    let mut compactness_vector = Vec::new();

    loop {
        if is_diverging(&compactness_vector, n) { break; }
        compactness_vector.push(estimate_space_compactness(&vector_space));
        tick(&mut vector_space, 1);
    };

    let (time, _, _) = compactness_vector.iter()
        .enumerate()
        .fold((0, 0, 0.0), |(pos, best_x, best_y), (i, (x, y))| {
            match i {
                0 => (i, *x, *y),
                _ => {
                    if x < &best_x && y > &best_y {
                        (i, *x, *y)
                    } else {
                        (pos, best_x, best_y)
                    }
                }
            }
        });

    time
}

fn is_diverging(compactness_vector: &Vec<(isize, f32)>, nlookback: usize) -> bool {
    let len = compactness_vector.len();

    match len < nlookback {
        true => false,
        false => {
            let last = compactness_vector[len - 1];
            let earlier = compactness_vector[len - nlookback];

            last.0 > earlier.0 && last.1 < earlier.1
        }
    }
}

fn estimate_space_compactness(vector_space: &Vec<Vector>) -> (isize, f32) {
    let (min, max) = find_min_and_max(vector_space);

    let y_compactness = max_same_y_coordinate_count(vector_space) as f32
        / (max.y - min.y) as f32;

    (max.x - min.x, y_compactness)
}

fn max_same_y_coordinate_count(vector_space: &Vec<Vector>) -> isize {
    let counts: HashMap<isize, isize> = vector_space.iter()
        .map(|vec| vec.coord.y)
        .fold(HashMap::new(), |mut map, y| {
            *map.entry(y).or_insert(0) += 1;
            map
        });

    *counts.values().max().unwrap()
}

fn draw_space(vector_space: &Vec<Vector>) {
    let (min, max) = find_min_and_max(vector_space);
    let normalized_space = transform_vector_space(vector_space, min);
    let empty_space = vec![vec![' '; (max.x - min.x) as usize + 1]; (max.y - min.y) as usize + 1];

    let space = normalized_space.iter()
        .fold(empty_space, |mut space, vec| {
            space[vec.coord.y as usize][vec.coord.x as usize] = '#';
            space
        });

    for row in space {
        let output: String = row.iter().collect();
        println!("{}", output);
    }
}

fn transform_vector_space(vector_space: &Vec<Vector>, new_origin: Point) -> Vec<Vector> {
    let transformed: Vec<Vector> = vector_space.iter()
        .map(|a| {
            Vector {
                coord: Point {
                    x: a.coord.x - new_origin.x,
                    y: a.coord.y - new_origin.y
                },
                i: a.i,
                j: a.j
            }
        })
        .collect();

    transformed
}

fn find_min_and_max(vector_space: &Vec<Vector>) -> (Point, Point) {
    let (min, max) = vector_space.iter()
        .fold(None, |min_max: Option<(Point, Point)>, a| {
            match min_max {
                Some((min, max)) => {
                    let min = Point {
                        x: cmp::min(min.x, a.coord.x),
                        y: cmp::min(min.y, a.coord.y)
                    };

                    let max = Point {
                        x: cmp::max(max.x, a.coord.x),
                        y: cmp::max(max.y, a.coord.y)
                    };

                    Some((min, max))
                },
                None => Some((a.coord, a.coord))
            }
        }).expect("");

    (min, max)
}

fn tick(vector_space: &mut Vec<Vector>, seconds: isize) {
    for vec in vector_space.iter_mut() {
        vec.coord.x += vec.i * seconds;
        vec.coord.y += vec.j * seconds;
    }
}

fn parse_line_to_tuple(line: &str) -> Vector {
    let re = Regex::new(r"position=<[ ]*([-]?[0-9]+),[ ]*([-]?[0-9]+)> velocity=<[ ]*([-]?[0-9]+),[ ]*([-]?[0-9]+)>").unwrap();

    let values = re.captures(line).unwrap();

    let parser = |i| {
        values.get(i)
            .map(|m| -> isize { m.as_str().parse().unwrap() })
            .unwrap()
    };

    Vector {
        coord: Point {
            x: parser(1),
            y: parser(2)
        },
        i: parser(3),
        j: parser(4)
    }
}
