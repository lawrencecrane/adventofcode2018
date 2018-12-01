use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let mut f = File::open("data/day_1_input")
        .expect("File not found");

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Something went wrong reading the file");

    let buffer = buffer;

    println!("Resulting frequency: {}", resulting_frequency(buffer.lines()));
    println!("First duplicate frequency: {}", first_duplicate_frequency(buffer.lines()));
}

fn first_duplicate_frequency(lines: std::str::Lines) -> i32 {
    // create infinite repetition of the buffer:
    let inf_lines = lines.cycle();
    // create a set to store seen frequencies:
    let mut seen_frequencies = HashSet::new();

    let mut sum = 0;
    seen_frequencies.insert(0);

    for number in inf_lines {
        sum += parse_signed_string(number);
        if seen_frequencies.contains(&sum) { break; }

        seen_frequencies.insert(sum);
    }

   sum
}

fn resulting_frequency(lines: std::str::Lines) -> i32 {
    let answer: i32 = lines.map(parse_signed_string).sum();

    answer
}

fn parse_signed_string(x: &str) -> i32 {
    let y: i32 = match x.parse() {
        Ok(n) => n,
        Err(_) => 0
    };

    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_signed_string() {
        assert_eq!(parse_signed_string("100"), 100);
        assert_eq!(parse_signed_string("+100"), 100);
        assert_eq!(parse_signed_string("-100"), -100);
    }

    #[test]
    fn test_first_duplicate_frequency() {
        assert_eq!(first_duplicate_frequency(String::from("+1\n-1").lines()), 0);
        assert_eq!(first_duplicate_frequency(String::from("+3\n+3\n+4\n-2\n-4").lines()), 10);
        assert_eq!(first_duplicate_frequency(String::from("-6\n+3\n+8\n+5\n-6").lines()), 5);
        assert_eq!(first_duplicate_frequency(String::from("+7\n+7\n-2\n-7\n-4").lines()), 14);
    }
}
