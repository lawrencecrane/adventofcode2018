extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::char;
use regex::Regex;

fn main() {
    let mut f = File::open("data/day_07_input").unwrap();

    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let order = instruction_order(buffer.lines());
    println!("Instruction order: {}", order);

    let total_time = time_to_complete_all_steps(buffer.lines(), 5, 60);
    println!("Total time to complete steps in parallel: {}", total_time);
}

#[derive(Debug)]
struct Tick {
    total_time: usize,
    time_penalty: usize,
    workers: Vec<(usize, usize)>,
    nworkers: usize
}

fn time_to_complete_all_steps(lines: std::str::Lines, workers: usize, time_penalty: usize) -> usize {
    let network = create_network(lines);

    let (result, _) = complete_all_parallel((Tick {
        total_time: 0,
        time_penalty: time_penalty,
        workers: vec![(0, 0); workers],
        nworkers: workers
    }, network));

    result.total_time
}

fn complete_all_parallel(xs: (Tick, (Vec<Vec<usize>>, HashSet<usize>))) -> (Tick, (Vec<Vec<usize>>, HashSet<usize>)) {
    let (mut x, mut network) = xs;

    let available: VecDeque<usize> = find_n_available_instructions(&network.0, &network.1, x.nworkers);
    // Give idle workers a new task if there is any available:
    let workers = assign_tasks(&x, available_tasks(available, &x));
    let min_completion_time = min_completion_time(&workers);

    let result = match min_completion_time {
        Some(min_time) => {
            // Substract the shortest completion time from tasks,
            // if any tasks finished, remove them:
            let workers: Vec<(usize, usize)> = workers.iter()
                .map(|(time, task)| {
                    match time == &0 {
                        true => (*time, *task),
                        false => {
                            let new_time = time - min_time;
                            match new_time == 0 {
                                true => {
                                    network.1.remove(task);
                                    (0, 0)
                                },
                                false => {
                                    (new_time, *task)
                                }
                            }
                        }
                    }
                })
                .collect();

            x.total_time += min_time;
            x.workers = workers;

            complete_all_parallel((x, network))
        },
        None => {
            (x, network)
        }
    };

    result
}

fn min_completion_time(workers: &Vec<(usize, usize)>) -> Option<usize> {
    workers.iter()
        .filter(|(time, _)| time != &0)
        .map(|(time, _)| *time)
        .min()
}

fn assign_tasks(x: &Tick, mut available: VecDeque<usize>) -> Vec<(usize, usize)> {
    x.workers.iter()
        .map(|(time, task)| {
            match time == &0 && task == &0 {
                true => {
                    let t = available.pop_front();
                    match t {
                        Some(n) => (1 + n + x.time_penalty, n),
                        None => (*time, *task)
                    }
                },
                false => (*time, *task)
            }
        })
        .collect()
}

fn available_tasks(available: VecDeque<usize>, current: &Tick) -> VecDeque<usize> {
    let current_tasks: Vec<usize> = current.workers.iter()
        .filter(|(time, _)| time > &0)
        .map(|(_, task)| *task)
        .collect();

    available
        .iter()
        .filter(|task| !current_tasks.contains(task))
        .map(|x| *x)
        .collect()
}

fn instruction_order(lines: std::str::Lines) -> String {
    let (network, valid) = create_network(lines);
    let order = find_instruction_order(&network, valid, Vec::new());

    order.iter().collect()
}

fn create_network(lines: std::str::Lines) -> (Vec<Vec<usize>>, HashSet<usize>) {
    let (network, valid) = lines
        .map(|x| {
            let (dependee, dependent) = parse_line_to_tuple(x);
            (transform_char(dependee), transform_char(dependent))
        } )
        .fold((vec![vec![0; 26]; 26], HashSet::new()),
              |(mut network, mut valid), (dependee, dependent)| {
                  valid.insert(dependee);
                  valid.insert(dependent);
                  network[dependent][dependee] = 1;

                  (network, valid)
              });

    (network, valid)
}

fn find_instruction_order(network: &Vec<Vec<usize>>,
                         mut valid: HashSet<usize>,
                         mut result: Vec<char>) -> Vec<char> {
    let available = find_n_available_instructions(network, &valid, 1);

    match available.len() {
         1 => {
            let c = char::from_u32(available[0] as u32 + 'A' as u32).unwrap();
            valid.remove(&available[0]);
            result.push(c);
            find_instruction_order(network, valid, result)
        },
        _ => result
    }
}

fn find_n_available_instructions(network: &Vec<Vec<usize>>, valid: &HashSet<usize>, n: usize) -> VecDeque<usize> {
    let available: VecDeque<usize> = network.iter()
        .enumerate()
        .filter(|(i, _)| valid.contains(i))
        .map(|(i, x)| -> (usize, usize) {
            let sum = x.iter()
                .enumerate()
                .filter(|(j, _)| valid.contains(j))
                .map(|(_, y)| y)
                .sum();

            (i, sum)
        })
        .filter(|(_, sum)| sum == &0)
        .map(|(i, _)| i)
        .take(n)
        .collect();

    available
}

fn transform_char(c: char) -> usize {
    c as usize - 'A' as usize
}

fn parse_line_to_tuple(line: &str) -> (char, char) {
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.")
        .unwrap();

    let values = re.captures(line).unwrap();

    let dependee = values.get(1)
        .map(|m| -> char { m.as_str().chars().next().unwrap() })
        .unwrap();

    let dependent = values.get(2)
        .map(|m| -> char { m.as_str().chars().next().unwrap() })
        .unwrap();

    (dependee, dependent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_order() {
        let input = String::from("Step C must be finished before step A can begin.\n\
                                  Step C must be finished before step F can begin.\n\
                                  Step A must be finished before step B can begin.\n\
                                  Step A must be finished before step D can begin.\n\
                                  Step B must be finished before step E can begin.\n\
                                  Step D must be finished before step E can begin.\n\
                                  Step F must be finished before step E can begin.");

        assert_eq!(instruction_order(input.lines()), String::from("CABDFE"));
    }

    #[test]
    fn test_time_to_complete_all_steps() {
        let input = String::from("Step C must be finished before step A can begin.\n\
                                  Step C must be finished before step F can begin.\n\
                                  Step A must be finished before step B can begin.\n\
                                  Step A must be finished before step D can begin.\n\
                                  Step B must be finished before step E can begin.\n\
                                  Step D must be finished before step E can begin.\n\
                                  Step F must be finished before step E can begin.");

        assert_eq!(time_to_complete_all_steps(input.lines(), 2, 0), 15);
    }
}
