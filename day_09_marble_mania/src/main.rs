extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() {
    let mut f = File::open("data/day_09_input").unwrap();

    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let (nplayers, nmarbles) = parse_line_to_tuple(&buffer);

    let highscore = game_result(nplayers, nmarbles);
    println!("Highscore: {}", highscore);

    let highscore_100x = game_result(nplayers, nmarbles*100);
    println!("Highscore: {}", highscore_100x);
}

fn game_result(nplayers: usize, nmarbles: usize) -> usize {
    let mut game_circle: Vec<Option<(usize, usize)>> = vec![None; nmarbles + 1];
    game_circle[0] = Some((1,1));
    game_circle[1] = Some((0,0));

    let (_, _, scores) = (1..nmarbles + 1).zip((0..nplayers).cycle())
        .fold((game_circle, 0, vec![0; nplayers]),
              |(mut circle, current_marble, mut scores), (marble, player)| {
                  // skip the first marble:
                  if marble == 1 { (circle, 1, scores) }
                  else {
                      match marble % 23 == 0 {
                          false => {
                              insert_one_clockwise_after(current_marble, marble, &mut circle);

                              (circle, marble, scores)
                          },
                          true => {
                              let seven_previous = find_marble_counter_clockwise(&circle,
                                                                                 current_marble,
                                                                                 7);

                              let next = remove_marble(seven_previous, &mut circle);
                              scores[player] += marble + seven_previous;

                              (circle, next, scores)
                          }
                      }
                  }
              });

    *scores.iter().max().unwrap()
}

fn remove_marble(marble: usize, game_circle: &mut Vec<Option<(usize, usize)>>) -> usize {
    let (next, prev) = game_circle[marble].unwrap();
    let (_, prevprev) = game_circle[prev].unwrap();
    let (nextnext, _) = game_circle[next].unwrap();

    game_circle[prev] = Some((next, prevprev));
    game_circle[next] = Some((nextnext, prev));
    game_circle[marble] = None;

    next
}

fn insert_one_clockwise_after(current: usize, marble: usize, game_circle: &mut Vec<Option<(usize, usize)>>) {
    let (next, _) = game_circle[current].unwrap();
    let (nextnext, nextprev) = game_circle[next].unwrap();
    let (nextnextnext, _) = game_circle[nextnext].unwrap();

    game_circle[marble] = Some((nextnext, next));
    game_circle[next] = Some((marble, nextprev));
    game_circle[nextnext] = Some((nextnextnext, marble));
}

fn find_marble_counter_clockwise(game_circle: &Vec<Option<(usize, usize)>>,
                                  current_marble: usize, n: usize) -> usize {
    (0..n).fold(current_marble, |current, _| {
        let (_, previous) = game_circle[current].unwrap();
        previous
    })
}

fn parse_line_to_tuple(line: &str) -> (usize, usize) {
    let re = Regex::new(r"([0-9]+) players; last marble is worth ([0-9]+) points").unwrap();
    let values = re.captures(line).unwrap();

    let nplayers = values.get(1)
        .map(|m| -> usize { m.as_str().parse().unwrap() })
        .unwrap();

    let nmarbles = values.get(2)
        .map(|m| -> usize { m.as_str().parse().unwrap() })
        .unwrap();

    (nplayers, nmarbles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_result() {
        assert_eq!(game_result(9, 25), 32);
        assert_eq!(game_result(10, 1618), 8317);
        assert_eq!(game_result(13, 7999), 146373);
        assert_eq!(game_result(17, 1104), 2764);
        assert_eq!(game_result(21, 6111), 54718);
        assert_eq!(game_result(30, 5807), 37305);
    }
}
