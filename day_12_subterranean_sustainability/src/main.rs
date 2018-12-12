extern crate regex;
// extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::collections::HashMap;
use regex::Regex;
// use itertools::Itertools;

fn main() {
    let mut f = File::open("data/day_12_input").unwrap();

    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let (generation, transformations) = parse_input(buffer);

    println!("After 20 gens, sum: {}",
             sum_pot_indices_with_plants_after_n_gens(generation.clone(),
                                                      &transformations, 20));

    println!("After 50000000000 gens, sum: {}",
             sum_pot_indices_with_plants_after_big_n_gens(generation,
                                                          &transformations, 50000000000));
}

// fn find_pattern_in_generations
fn sum_pot_indices_with_plants_after_big_n_gens(initial_gen: VecDeque<usize>,
                                                trans: &Vec<(Vec<usize>, usize)>,
                                                n: isize) -> isize {
    let (mut generation, mut zero_idx) = balance_generation(initial_gen);
    let (mut seen, mut sum_prev, mut sum_diff, mut ngen) = (HashMap::new(), 0, 0, 0);
    let mut snd_seen = false;

    for g in 1..n + 1 {
        let (gen, idx) = next_generation(generation, trans, zero_idx);

        let indices = get_plant_indices(&gen, idx);
        let sum: isize = indices.iter().sum();
        let indices_norm: Vec<isize> = indices.iter().map(|x| x - indices[0]).collect();

        if seen.contains_key(&indices_norm) {
            // loop once more to get the constant difference:
            if snd_seen {
                sum_diff = sum - sum_prev;
                ngen = g;
                sum_prev = sum;

                break
            }

            snd_seen = true;
        } else {
            seen.insert(indices_norm, (n, sum, indices[0]));
        }

        generation = gen;
        zero_idx = idx;
        sum_prev = sum;
    };

    match ngen {
        0 => sum_prev,
        _ => sum_prev + (n - ngen) * sum_diff
    }
}

fn sum_pot_indices_with_plants_after_n_gens(initial_gen: VecDeque<usize>,
                                            trans: &Vec<(Vec<usize>, usize)>,
                                            n: usize) -> isize {
    let (init_gen, zero_idx) = balance_generation(initial_gen);

    let (gen, zero_idx) = (0..n)
        .fold((init_gen, zero_idx), |(gen, idx), _| {
            let (gen, idx) = next_generation(gen, trans, idx);
            (gen, idx)
    });

    let indices = get_plant_indices(&gen, zero_idx);
    indices.iter().sum()
}

fn get_plant_indices(gen: &Vec<usize>, zero_idx: usize) -> Vec<isize> {
    gen.iter().enumerate()
        .filter(|(_, x)| x == &&1)
        .map(|(i, _)| i as isize - zero_idx as isize)
        .collect()
}

// we could do this more efficiently (and just store the indices etc.)
// but it's not really needed:
fn next_generation(current_gen: Vec<usize>,
                   transformations: &Vec<(Vec<usize>, usize)>,
                   zero_idx: usize) -> (Vec<usize>, usize) {
    let plant_idxs = (0..current_gen.len() - 4)
        .fold(Vec::new(), |plant_idxs, i| {
            let s: Vec<usize> = current_gen.get(i..i+5).unwrap().to_vec();

            transformations.iter()
                .filter(|(_, res)| res == &1)
                .fold(plant_idxs, |mut idxs, (trans, _)| {
                    if trans == &s { idxs.push(i + 2); }
                    idxs
                })
        });

    let mut new_gen = vec![0; 8 + (1 + plant_idxs[plant_idxs.len() - 1] - plant_idxs[0])];
    let offset = 4 - plant_idxs[0] as isize;

    for i in plant_idxs {
        new_gen[(i as isize + offset) as usize] = 1;
    };

    (new_gen, (zero_idx as isize + offset) as usize)
}

// doesn't care if many zeros in front/back...
fn balance_generation(mut gen: VecDeque<usize>) -> (Vec<usize>, usize) {
    let first = gen.iter()
        .position(|x| x == &1).unwrap();

    let zero_idx = (4 as usize).checked_sub(first).unwrap_or(0);

    let last = gen.iter()
        .rposition(|x| x == &1).unwrap();

    let len = gen.len();

    if zero_idx > 0 {
        for _ in 0..zero_idx {
            gen.push_front(0);
        }
    }

    if last > len - 5 {
        for _ in (len - 5)..last {
            gen.push_back(0);
        }
    }

    (gen.iter().map(|x| *x).collect(), zero_idx)
}

fn parse_input(buffer: String) -> (VecDeque<usize>, Vec<(Vec<usize>, usize)>) {
    let mut lines = buffer.lines();
    let initial_state = parse_initial_state(lines.next().unwrap());

    lines.next();
    let transformations = lines
        .map(parse_transformation)
        .collect();

    (initial_state, transformations)
}

fn parse_transformation(input: &str) -> (Vec<usize>, usize) {
    let re = Regex::new(r"([.#]*) => ([.#])").unwrap();
    let re_capture = re.captures(input).unwrap();

    let transformation = re_capture
        .get(1)
        .map(|m| {
            String::from(m.as_str())
        })
        .unwrap();

    let result = re_capture
        .get(2)
        .map(|m| {
            m.as_str().chars().next().unwrap()
        })
        .unwrap();

    let transformation = transformation.chars()
        .map(|c| {
            plant_state_to_int(c)
        })
        .collect();

    (transformation, plant_state_to_int(result))
}

fn parse_initial_state(input: &str) -> VecDeque<usize> {
    let re = Regex::new(r"initial state: ([.#]*)").unwrap();
    let re_capture: String = re.captures(input).unwrap()
        .get(1)
        .map(|m| {
            String::from(m.as_str())
        })
        .unwrap();

    let initial_state = re_capture.chars()
        .map(|c| {
            plant_state_to_int(c)
        })
        .collect();

    initial_state
}

fn plant_state_to_int(c: char) -> usize {
    match c {
        '#' => 1,
        _  => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "initial state: #..#.#..##......###...###\n\
                         \n\
                         ...## => #\n\
                         ..#.. => #\n\
                         .#... => #\n\
                         .#.#. => #\n\
                         .#.## => #\n\
                         .##.. => #\n\
                         .#### => #\n\
                         #.#.# => #\n\
                         #.### => #\n\
                         ##.#. => #\n\
                         ##.## => #\n\
                         ###.. => #\n\
                         ###.# => #\n\
                         ####. => #";

    #[test]
    fn test_sum_pot() {
        let (generation, transformations) = parse_input(String::from(INPUT));

        assert_eq!(sum_pot_indices_with_plants_after_n_gens(
            generation,
            &transformations,
            20), 325);
    }
 
    #[test]
    fn test_sum_pot_big() {
        let (generation, transformations) = parse_input(String::from(INPUT));

        assert_eq!(sum_pot_indices_with_plants_after_big_n_gens(
            generation.clone(),
            &transformations,
            20), 325);

        assert_eq!(sum_pot_indices_with_plants_after_big_n_gens(generation.clone(),
                                                                &transformations,
                                                                200),
                   sum_pot_indices_with_plants_after_n_gens(generation.clone(),
                                                            &transformations,
                                                            200));
    }
}
