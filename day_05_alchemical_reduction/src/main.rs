use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("data/day_05_input")
        .expect("File not found");

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Something went wrong reading the file");

    let buffer = buffer.replace('\n', "");

    let reduced_polymer = reduce_polymer(&buffer);
    let rereduced_polymer = reduce_polymer(&reduced_polymer);
    println!("Reducer works? {}", reduced_polymer == rereduced_polymer);
    println!("Reduced polymer units: {}", reduced_polymer.len());
}

fn reduce_polymer(polymer: &str) -> String {
    let reduced = polymer.chars()
        // .map(|c| c as isize)
        .fold(Vec::new(), |mut polymer: Vec<char>, c| {
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
        });

    let reduced: String = reduced.iter().collect();

    reduced
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_polymer() {
        assert_eq!(reduce_polymer("dabAcCaCBAcCcaDA"), String::from("dabCBAcaDA"));
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
