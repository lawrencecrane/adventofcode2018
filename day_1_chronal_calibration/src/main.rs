use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("data/day_1_input")
        .expect("File not found");

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Something went wrong reading the file");

    let frequency: i32 = buffer.lines()
        .map(parse_signed_string)
        .sum();

    println!("{}", frequency);
}

fn parse_signed_string(x: &str) -> i32 {
    let y: String = x.chars().skip_while(|&c| c == '+').collect();
    let y: i32 = match y.parse() {
        Ok(n) => n,
        Err(_) => 0
    };

    y
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_signed_string() {
        assert_eq!(parse_signed_string("100"), 100);
        assert_eq!(parse_signed_string("+100"), 100);
        assert_eq!(parse_signed_string("-100"), -100);
    }
}
