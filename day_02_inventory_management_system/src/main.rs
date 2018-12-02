extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

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
    let mut seen_one_char_masked_ids = HashSet::new();

    let masked_ids = lines.flat_map(generate_one_char_masked_ids);

    for masked_id in masked_ids {
        if seen_one_char_masked_ids.contains(&masked_id) {
            return Some(masked_id.replace("_", ""));
        }

        seen_one_char_masked_ids.insert(masked_id);
    }

    None
}

fn generate_one_char_masked_ids(id: &str) -> Vec<String> {
    let masked_ids: Vec<String> = (0..id.len())
        .map(|i| {
            format!("{}_{}", &id[0..i], &id[i+1..id.len()])
        })
        .collect();

    masked_ids
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
    // we could also just use hashmap...
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
