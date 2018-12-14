extern crate num_complex;

use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::HashMap;
use num_complex::Complex;

fn main() {
    let mut f = File::open("data/input").unwrap();

    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let (track, mut carts) = parse_to_track(&buffer);
    println!("{:?}", fst_crash_location(&track, &mut carts));

    let (track, carts) = parse_to_track(&buffer);
    println!("{:?}", last_cart_location(&track, carts));
}

fn last_cart_location(track: &HashMap<(i32, i32), char>, mut carts: Vec<Cart>) -> (i32, i32) {
    loop {
        carts.sort_by(|a, b| {
            match a.y.cmp(&b.y) {
                Ordering::Equal => a.x.cmp(&b.x),
                other => other,
            }});

        let mut locations: HashMap<(i32, i32), usize> = carts.iter()
            .enumerate()
            .fold(HashMap::new(), |mut locs, (i, cart)| {
                locs.insert((cart.x, cart.y), i);
                locs
            }
            );

        let mut to_be_deleted = HashSet::new();

        for cart in carts.iter_mut() {
            let idx = locations.remove(&(cart.x, cart.y));

            // I have been removed:
            if idx == None { continue; }

            cart.step();

            match track.get(&(cart.x, cart.y)) {
                Some('/') => { cart.turn('/') },
                Some('\\') => { cart.turn('\\') },
                Some('+') => { cart.turn_in_intersection() },
                _ => {}
            }

            if locations.contains_key(&(cart.x, cart.y)) {
                let idx_other = locations.remove(&(cart.x, cart.y));
                to_be_deleted.insert(idx.unwrap());
                to_be_deleted.insert(idx_other.unwrap());
            } else {
                locations.insert((cart.x, cart.y), idx.unwrap());
            }
        }

        carts = carts.iter()
            .enumerate()
            .filter(|(i, _)| !to_be_deleted.contains(i))
            .map(|(_, cart)| *cart)
            .collect();

        if carts.len() == 1 { break (carts[0].x, carts[0].y) }
    }
}

fn fst_crash_location(track: &HashMap<(i32, i32), char>, carts: &mut Vec<Cart>) -> (i32, i32) {
    loop {
        let position = tick(track, carts);
        if position != None { break position.unwrap() }
    }
}

fn tick(track: &HashMap<(i32, i32), char>, carts: &mut Vec<Cart>) -> Option<(i32, i32)> {
    let mut locations: HashSet<(i32, i32)> = carts.iter()
        .map(|cart| (cart.x, cart.y)).collect();

    carts.sort_by(|a, b| {
        match a.y.cmp(&b.y) {
            Ordering::Equal => a.x.cmp(&b.x),
            other => other,
        }});

    for cart in carts.iter_mut() {
        locations.remove(&(cart.x, cart.y));
        cart.step();

        match track.get(&(cart.x, cart.y)) {
            Some('/') => { cart.turn('/') },
            Some('\\') => { cart.turn('\\') },
            Some('+') => { cart.turn_in_intersection() },
            _ => {}
        }

        if locations.contains(&(cart.x, cart.y)) { return Some((cart.x, cart.y)) }

        locations.insert((cart.x, cart.y));
    }

    None
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Turn {
    LEFT,
    RIGHT,
    AHEAD
}

#[derive(Debug, Clone, Copy)]
struct Cart {
    x: i32,
    y: i32,
    direction: Complex<i32>,
    next_turn: Turn
}

impl Cart {
    fn turn_in_intersection(&mut self) {
        let turn = self.next_turn;
        match turn {
            Turn::LEFT  => {
                self.turn_left();
                self.next_turn = Turn::AHEAD;
            },
            Turn::AHEAD => { self.next_turn = Turn::RIGHT; },
            Turn::RIGHT => {
                self.turn_right();
                self.next_turn = Turn::LEFT;
            },
        }
    }

    fn step(&mut self) {
        match self.direction.re.abs() {
            1 => { self.x += self.direction.re; },
            _ => { self.y += self.direction.im; }
        }
    }

    fn turn(&mut self, track: char) {
        match track {
            '\\' => {
                match self.direction.re.abs() {
                    1 => self.turn_right(),
                    _ => self.turn_left()
                }
            },
            '/' => {
                match self.direction.re.abs() {
                    1 => self.turn_left(),
                    _ => self.turn_right()
                }
            },
            _ => { }
        }
    }

    fn turn_right(&mut self) {
        self.direction *= Complex::i();
    }

    fn turn_left(&mut self) {
        self.direction *= -1 * Complex::i();
    }
}

fn parse_to_track(raw: &String) -> (HashMap<(i32, i32), char>, Vec<Cart>) {
    let (track, carts) = raw.lines().enumerate()
        .fold((HashMap::new(), Vec::new()), |(track, carts), (y, line)| {
            line.chars().enumerate()
                .filter(|(_, c)| c != &' ')
                .fold((track, carts), |(mut track, mut carts), (x, c)| {
                    let x = x as i32;
                    let y = y as i32;

                    match c {
                        '^' => {
                            track.insert((x, y), '|');
                            carts.push(Cart { x: x, y: y,
                                              direction: -1 * Complex::i(),
                                              next_turn: Turn::LEFT });
                        },
                        'v' => {
                            track.insert((x, y), '|');
                            carts.push(Cart { x: x, y: y,
                                              direction: Complex::i(),
                                              next_turn: Turn::LEFT });
                        },
                        '<' => {
                            track.insert((x, y), '-');
                            carts.push(Cart { x: x, y: y,
                                              direction: Complex::new(-1, 0),
                                              next_turn: Turn::LEFT });
                        },
                        '>' => {
                            track.insert((x, y), '-');
                            carts.push(Cart { x: x, y: y,
                                              direction: Complex::new(1, 0),
                                              next_turn: Turn::LEFT });
                        },
                        other => { track.insert((x, y), other); }
                    }

                    (track, carts)
                })
        });

    (track, carts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fst_crash() {
        let mut f = File::open("data/input_test").unwrap();

        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();

        let (track, mut carts) = parse_to_track(&buffer);

        assert_eq!(fst_crash_location(&track, &mut carts), (7, 3));
    }

    #[test]
    fn test_last_cart() {
        let mut f = File::open("data/input_test2").unwrap();

        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();

        let (track, carts) = parse_to_track(&buffer);

        assert_eq!(last_cart_location(&track, carts), (6, 4));
    }
}
