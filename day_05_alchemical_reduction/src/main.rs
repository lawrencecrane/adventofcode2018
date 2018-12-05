use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let mut f = File::open("data/day_05_input")
        .expect("File not found");

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Something went wrong reading the file");

    let buffer = buffer.replace('\n', "");

    let reduced_polymer = reduce_polymer(&buffer);
    // let rereduced_polymer = reduce_polymer(&reduced_polymer);
    // println!("Reducer works? {}", reduced_polymer == rereduced_polymer);
    println!("Reduced polymer units: {}", reduced_polymer.len());

    let shortest_reduced_polymer = shortest_polymer(&buffer);
    println!("Shortest reduced polymer units: {}", shortest_reduced_polymer.len());
}

fn shortest_polymer(polymer: &str) -> String {
    let alphabet: HashSet<char> = polymer.chars()
        .map(|x| x.to_ascii_lowercase())
        .collect();

    let reduced_best = alphabet.iter()
        .map(|&letter| {
            polymer.chars()
                .filter(|c| c.to_ascii_lowercase() != letter)
                .fold(Vec::new(), polymer_reducer)
        })
        .fold(None, |best: Option<Vec<char>>, reduced| {
            match best {
                Some(v) => {
                    if reduced.len() < v.len() { Some(reduced) }
                    else { Some(v) }
                },
                None => Some(reduced)
            }
        }).expect("");

    let reduced_best: String = reduced_best.iter().collect();

    reduced_best
}

fn reduce_polymer(polymer: &str) -> String {
    let reduced = polymer.chars()
        .fold(Vec::new(), polymer_reducer);

    let reduced: String = reduced.iter().collect();

    reduced
}

fn polymer_reducer(mut polymer: Vec<char>, c: char) -> Vec<char> {
    let previous = polymer.pop();

    match previous {
        Some(n) => match ((n as isize) - (c as isize)).abs() {
            32 => {},
            _ => {
                polymer.push(n);
                polymer.push(c);
            }
        },
        None => {
            polymer.push(c);
        }
    };

    polymer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_polymer() {
        assert_eq!(reduce_polymer("dabAcCaCBAcCcaDA"), String::from("dabCBAcaDA"));
    }

    #[test]
    fn test_shortest_polymer() {
        assert_eq!(shortest_polymer("dabAcCaCBAcCcaDA"), String::from("daDA"));
    }

    #[test]
    fn ascii_32_property() {
        let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                     'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

        let lower_upper_property = chars.iter()
            .map(|&c| (c as isize) - (c.to_ascii_uppercase() as isize))
            .all(|d| d == 32);

        assert!(lower_upper_property);
    }
}
