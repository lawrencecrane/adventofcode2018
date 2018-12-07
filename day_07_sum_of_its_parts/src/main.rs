extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
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

fn time_to_complete_all_steps(lines: std::str::Lines, workers: usize, time_penalty: usize) -> usize {
    let network = create_depnet(lines);

    complete_all_parallel(vec![(0, ' '); workers], workers, network, time_penalty, 0)
}

fn complete_all_parallel(workers: Vec<(usize, char)>,
                         nworkers: usize,
                         mut network: HashMap<char, HashSet<char>>,
                         time_penalty: usize,
                         total_time: usize) -> usize {
    let available: VecDeque<char> = find_n_available_instructions(&network, nworkers);
    // Give idle workers a new task if there is any available:
    let workers = assign_tasks(&workers, available_tasks(available, &workers), time_penalty);
    let min_completion_time = min_completion_time(&workers);

    match min_completion_time {
        Some(min_time) => {
            // Substract the shortest completion time from tasks,
            // if any tasks finished, remove them:
            let workers: Vec<(usize, char)> = workers.iter()
                .map(|(time, task)| {
                    match time == &0 {
                        true => (*time, *task),
                        false => {
                            let new_time = time - min_time;
                            match new_time == 0 {
                                true => {
                                    remove_task(&mut network, &task);
                                    (0, ' ')
                                },
                                false => {
                                    (new_time, *task)
                                }
                            }
                        }
                    }
                })
                .collect();

            complete_all_parallel(workers, nworkers, network, time_penalty, total_time + min_time)
        },
        None => {
            total_time
        }
    }
}

fn min_completion_time<T>(workers: &Vec<(usize, T)>) -> Option<usize> {
    workers.iter()
        .filter(|(time, _)| time != &0)
        .map(|(time, _)| *time)
        .min()
}

fn assign_tasks(workers: &Vec<(usize, char)>, mut available: VecDeque<char>, time_penalty: usize) -> Vec<(usize, char)> {
    workers.iter()
        .map(|(time, task)| {
            match time == &0 {
                true => {
                    let t = available.pop_front();
                    match t {
                        Some(n) => (char_to_seconds(&n) + time_penalty, n),
                        None => (*time, *task)
                    }
                },
                false => (*time, *task)
            }
        })
        .collect()
}

fn available_tasks(available: VecDeque<char>, workers: &Vec<(usize, char)>) -> VecDeque<char> {
    let current_tasks: Vec<char> = workers.iter()
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
    let network = create_depnet(lines);
    let order = find_instruction_order(network, Vec::new());

    order.iter().collect()
}

fn find_instruction_order(mut network: HashMap<char, HashSet<char>>,
                          mut result: Vec<char>) -> Vec<char> {
    let available = find_n_available_instructions(&network, 1);

    match available.len() {
         1 => {
             let done = available[0];

             remove_task(&mut network, &done);
             result.push(done);
             find_instruction_order(network, result)
        },
        _ => result
    }
}

fn find_n_available_instructions(network: &HashMap<char, HashSet<char>>, n: usize) -> VecDeque<char> {
    let mut available: Vec<char> = network.iter()
        .filter(|(_, dependencies)| dependencies.len() == 0)
        .map(|(task, _)| *task)
        .collect::<Vec<char>>();

    available.sort();

    available
        .iter()
        .map(|x| *x)
        .take(n)
        .collect()
}

fn create_depnet(lines: std::str::Lines) -> HashMap<char, HashSet<char>> {
    let depnet = lines.
        fold(HashMap::new(), |mut map: HashMap<char, HashSet<char>>, x| {
            let (dependee, dependent) = parse_line_to_tuple(x);

            map.entry(dependee)
                .or_insert(HashSet::new());

            map.entry(dependent)
                .and_modify(|v| {
                    v.insert(dependee);
                })
                .or_insert({
                    let mut dependencies = HashSet::new();
                    dependencies.insert(dependee);
                    dependencies
                });

            map
        });

    depnet
}

fn remove_task(network: &mut HashMap<char, HashSet<char>>, task: &char) {
    for dependent in network.values_mut() {
        dependent.remove(task);
    }

    network.remove(task);
}

fn char_to_seconds(c: &char) -> usize {
    1 + ((*c as usize) - 'A' as usize)
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

    const INPUT: &str = "Step C must be finished before step A can begin.\n\
                         Step C must be finished before step F can begin.\n\
                         Step A must be finished before step B can begin.\n\
                         Step A must be finished before step D can begin.\n\
                         Step B must be finished before step E can begin.\n\
                         Step D must be finished before step E can begin.\n\
                         Step F must be finished before step E can begin.";

    #[test]
    fn test_instruction_order() {
        assert_eq!(instruction_order(String::from(INPUT).lines()), String::from("CABDFE"));
    }

    #[test]
    fn test_time_to_complete_all_steps() {
        assert_eq!(time_to_complete_all_steps(String::from(INPUT).lines(), 2, 0), 15);
    }
}
