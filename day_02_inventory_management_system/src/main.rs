extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use itertools::Itertools;

fn main() {
    let mut f = File::open("data/day_02_input")
        .expect("File not found");

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Something went wrong reading the file");

    println!("The checksum is {}", checksum(buffer.lines()));
    println!("The correct box id is {}", correct_box_id(buffer.lines()).expect(""));
}

fn correct_box_id(lines: std::str::Lines) -> Option<String> {
    let all_combinations = lines.combinations(2);

    for combination in all_combinations {
        if edit_distance(combination[0], combination[1]) == Some(1) {
            return Some(deduce_id(combination[0], combination[1]));
        }
    }

    None
}

fn deduce_id(x: &str, y: &str) -> String {
    let id: String = x.chars().zip(y.chars())
        .filter(|r| r.0 == r.1)
        .map(|r| r.0)
        .collect();

    id
}

fn edit_distance(x: &str, y: &str) -> Option<usize> {
    if x.len() != y.len() { return None; }

    let distance: usize = x.chars().zip(y.chars())
        .filter(|r| r.0 != r.1)
        .map(|_| 1)
        .sum();

    Some(distance)
}

fn checksum(lines: std::str::Lines) -> usize {
    let answer = lines
        .map(check_two_and_three_same_chars)
        .fold((0,0), |sum, val| {
            (sum.0 + val.0, sum.1 + val.1)
        });

    answer.0 * answer.1
}


fn check_two_and_three_same_chars(x: &str) -> (usize, usize) {
    // Let's assume that the characters are in unicode basic latin charset:
    let mut char_counts = [0; 128];

    for c in x.chars() {
        char_counts[c as usize] += 1;
    }

    let two = match char_counts.iter().find(|&&x| x == 2) {
        Some(_) => 1,
        None => 0
    };

    let three = match char_counts.iter().find(|&&x| x == 3) {
        Some(_) => 1,
        None => 0
    };

    (two, three)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        assert_eq!(checksum(
            String::from("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab").lines()),
                   12);
    }

    #[test]
    fn test_correct_box_id() {
        assert_eq!(correct_box_id(
            String::from("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz").lines()),
                   Some(String::from("fgij")));
    }
}
