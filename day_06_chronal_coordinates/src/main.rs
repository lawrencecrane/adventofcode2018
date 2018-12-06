extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::cmp;
use itertools::Itertools;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let mut f = File::open("data/day_06_input")
        .expect("File not found");

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Something went wrong reading the file");

    let coordinates = parse_to_points(buffer.lines());

    let largest_area = largest_finite_area(&coordinates);
    let area_with_distance_10000 = area_with_total_distance(&coordinates, 10000);

    println!("Largest finite area: {}", largest_area);
    println!("Area with total distance under 10000: {}", area_with_distance_10000);
}

fn area_with_total_distance(points: &Vec<Point>, upperlimit: usize) -> usize {
    let (min, max) = find_min_and_max_point(&points);
    let transformed_points = transform_points(&points, min);

    let space = map_total_distance(&transformed_points,
                                   1 + max.x as usize,
                                   1 + max.y as usize);

    space.iter()
        .flat_map(|x| x)
        .filter(|&dist| dist < &upperlimit)
        .count()
}

fn largest_finite_area(points: &Vec<Point>) -> usize {
    let (min, max) = find_min_and_max_point(&points);
    let transformed_points = transform_points(&points, min);

    let space = map_closest_points(&transformed_points,
                                   1 + max.x as usize,
                                   1 + max.y as usize);

    let boundary_ids = boundary_points(&space);

    let areas: HashMap<usize, usize> = space.iter()
        .flat_map(|x| x)
        .filter(|&point| {
            match point {
                Some(id) => !boundary_ids.contains(id),
                None => false
            }
        })
        .fold(HashMap::new(), |mut counter, point| {
            *counter.entry(point.expect("")).or_insert(0) += 1;

            counter
        });

    *areas.values().max().expect("")
}

fn boundary_points(space: &Vec<Vec<Option<usize>>>) -> HashSet<usize> {
    // bottom and top row:
    let boundary_ids = space[0].iter()
        .chain(space[space.len() - 1].iter())
        .fold(HashSet::new(), |mut set, id| {
            match id {
                Some(n) => {
                    set.insert(*n);
                    set
                },
                None => set
            }
        });

    // left and right column:
    let boundary_ids = space.iter().flat_map(|row| {
        vec![row[0], row[row.len() - 1]]
    }).fold(boundary_ids, |mut set, id| {
        match id {
            Some(n) => {
                set.insert(n);
                set
            },
            None => set
        }
    });

    boundary_ids
}

fn map_closest_points(points: &Vec<Point>, rows: usize, cols: usize) -> Vec<Vec<Option<usize>>> {
    let space = (0..cols).cartesian_product(0..rows)
        .map(|(i, j)| {
            Point {
                id: 0,
                x: i as isize,
                y: j as isize
            }
        })
        .fold(vec![vec![None; cols]; rows], |mut space, a| {
            space[a.y as usize][a.x as usize] = closest_point(&a, points);
            space
        });

    space
}

fn map_total_distance(points: &Vec<Point>, rows: usize, cols: usize) -> Vec<Vec<usize>> {
    let space = (0..cols).cartesian_product(0..rows)
        .map(|(i, j)| {
            Point {
                id: 0,
                x: i as isize,
                y: j as isize
            }
        })
        .fold(vec![vec![0; cols]; rows], |mut space, a| {
            space[a.y as usize][a.x as usize] = total_distance_to_all(&a, points);
            space
        });

    space
}

fn transform_points(points: &Vec<Point>, new_origin: Point) -> Vec<Point> {
    let transformed: Vec<Point> = points.iter()
        .map(|a| {
            Point {
                id: a.id,
                x: a.x - new_origin.x,
                y: a.y - new_origin.y
            }
        })
        .collect();

    transformed
}

fn find_min_and_max_point(points: &Vec<Point>) -> (Point, Point) {
    let (min, max) = points.iter()
        .fold(None, |min_max: Option<(Point, Point)>, a| {
            match min_max {
                Some((min, max)) => {
                    let min = Point {
                        id: 0,
                        x: cmp::min(min.x, a.x),
                        y: cmp::min(min.y, a.y)
                    };

                    let max = Point {
                        id: 0,
                        x: cmp::max(max.x, a.x),
                        y: cmp::max(max.y, a.y)
                    };

                    Some((min, max))
                },
                None => Some((*a, *a))
            }
        }).expect("");

    (min, max)
}

fn total_distance_to_all(a: &Point, points: &Vec<Point>) -> usize {
    let total = points.iter()
        .map(|b| taxicab_distance(a, b) as usize)
        .sum();

    total
}

fn closest_point(a: &Point, points: &Vec<Point>) -> Option<usize> {
    let (id, _, count) = points.iter()
        .fold(None, |closest, b| {
            let distance = taxicab_distance(a, b);

            match closest {
                Some ((_, dist, _)) if distance < dist => Some((b.id, distance, 1)),
                Some ((c, dist, count)) if distance == dist => Some((c, dist, count + 1)),
                Some ((c, dist, count)) => Some((c, dist, count)),
                None => Some((b.id, distance, 1))
            }
        }).expect("");

    match count {
        1 => Some(id),
        _ => None
    }
}

fn taxicab_distance(a: &Point, b: &Point) -> isize {
    (b.x - a.x).abs() + (b.y - a.y).abs()
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    id: usize,
    x: isize,
    y: isize
}

fn parse_to_points(lines: std::str::Lines) -> Vec<Point> {
    let points: Vec<Point> = lines.enumerate()
        .map(|(i, x)| string_to_point(x, i+1))
        .collect();

    points
}

fn string_to_point(x: &str, id: usize) -> Point {
    let xs: Vec<isize> = x.split(", ")
        .map(|coord| coord.parse().expect(""))
        .collect();

    Point {
        id: id,
        x: xs[0],
        y: xs[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_finite_area() {
        let input = parse_to_points(
            String::from("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9").lines()
        );

        assert_eq!(largest_finite_area(&input), 17);
    }

    #[test]
    fn test_area_with_total_distance() {
        let input = parse_to_points(
            String::from("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9").lines()
        );

        assert_eq!(area_with_total_distance(&input, 32), 16);
    }
}
