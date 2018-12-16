extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;

const OPTCODES: [&Fn(&Vec<usize>, &Vec<usize>) -> Vec<usize>; 16] = [
    &addr,
    &addi,
    &mulr,
    &muli,
    &banr,
    &bani,
    &borr,
    &boni,
    &setr,
    &seti,
    &gtir,
    &gtri,
    &gtrr,
    &eqir,
    &eqri,
    &eqrr
];

fn main() {
    let mut f = File::open("data/input").unwrap();

    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let splitted: Vec<&str> = buffer.split("\n\n\n").collect();

    let sample_count = count_samples(splitted[0]);
    println!("Count of samples that behave like three or more opcodes: {:?}", sample_count);

    let mappings = map_optcodes_to_fun(splitted[0]);
    let program = splitted[1];
    let output = execute_program(program, mappings);

    println!("Register after running the program: {:?}", output);
}

fn execute_program(program: &str, optcode_mapping: HashMap<usize, usize>) -> Vec<usize> {
    let output = program.lines()
        .filter(|line| line.len() != 0)
        .fold(vec![0, 0, 0, 0], |out, line| {
            let inst: Vec<usize> = line.split(' ').map(|x| -> usize { x.parse().unwrap() }).collect();
            let mapping = optcode_mapping.get(&inst[0]).unwrap();
            OPTCODES[*mapping](&inst, &out)
        });

    output
}

fn map_optcodes_to_fun(samples: &str) -> HashMap<usize, usize> {
    let mut matches: HashMap<usize, HashSet<usize>> = samples.split("\n\n")
        .fold(HashMap::new(), |mut matches, x| {
            let (reg, inst, reg_cmp) = parse_sample(x);
            matching_optcodes(inst, reg, reg_cmp, &mut matches);
            matches
        });

    let mut mappings = Vec::new();

    loop {
        let mapping = find_valid_mapping(&mut matches);
        if mapping == None { break }
        mappings.push(mapping.unwrap());
    }

    let mappings = mappings.iter()
        .fold(HashMap::new(), |mut map, (f, opt)| {
            map.insert(*opt, *f);
            map
        });

    mappings
}

fn find_valid_mapping(matches: &mut HashMap<usize, HashSet<usize>>) -> Option<(usize, usize)> {
    let mapping = matches.iter()
        .find(|(_, opts)| opts.len() == 1)
        .map(|(key, val)| {
            let opt = val.iter().next().unwrap();
            (*key, *opt)
        });

    match mapping {
        None => None,
        Some((fun, opt)) => {
            matches.remove(&fun);
            for (_, opts) in matches.iter_mut() { opts.remove(&opt); }

            mapping
        }
    }
}

fn matching_optcodes(inst: Vec<usize>, reg: Vec<usize>, reg_cmp: Vec<usize>,
                     opt_matches: &mut HashMap<usize, HashSet<usize>>) {
    OPTCODES.iter()
        .enumerate()
        .fold(opt_matches, |matches, (i, f)| {
            if f(&inst, &reg) == reg_cmp {
                // matches
                let matching = matches.entry(i).or_insert(HashSet::new());
                matching.insert(inst[0]);
            }

            matches
        });
}

fn count_samples(samples: &str) -> usize {
    samples.split("\n\n")
        .map(|x| {
            let (reg, inst, reg_cmp) = parse_sample(x);
            matching_optcode_count(inst, reg, reg_cmp)
        })
        .filter(|x| x >= &3)
        .count()
}

fn matching_optcode_count(inst: Vec<usize>, reg: Vec<usize>, reg_cmp: Vec<usize>) -> usize {
    OPTCODES.iter()
        .fold(0, |count, f| {
            count + (f(&inst, &reg) == reg_cmp) as usize
        })
}

fn parse_sample(x: &str) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let re = Regex::new(r"Before: \[(.*)\]\n(.*)\nAfter:  \[(.*)\]").unwrap();
    let re_capture = re.captures(x).unwrap();

    let parser = |i, delim| {
        re_capture.get(i)
            .map(|m| -> Vec<usize> {
                m.as_str().split(delim)
                    .map(|x| -> usize {
                        x.parse().unwrap()
                    })
                    .collect()
            })
            .unwrap()
    };

    let reg_before = parser(1, ", ");
    let instruction = parser(2, " ");
    let reg_after = parser(3, ", ");

    (reg_before, instruction, reg_after)
}

/// stores into register C the result of given function on register A and register B.
fn instrr(inst: &Vec<usize>, reg: &Vec<usize>, fun: &Fn(usize, usize) -> usize)
          -> Vec<usize> {
    let mut new_reg = reg.clone();
    new_reg[inst[3]] = fun(reg[inst[1]], reg[inst[2]]);
    new_reg
}

/// stores into register C the result of given function on register A and value B.
fn instri(inst: &Vec<usize>, reg: &Vec<usize>, fun: &Fn(usize, usize) -> usize)
          -> Vec<usize> {
    let mut new_reg = reg.clone();
    new_reg[inst[3]] = fun(reg[inst[1]], inst[2]);
    new_reg
}

/// stores into register C the result of given function on register A and value B.
fn instir(inst: &Vec<usize>, reg: &Vec<usize>, fun: &Fn(usize, usize) -> usize)
          -> Vec<usize> {
    let mut new_reg = reg.clone();
    new_reg[inst[3]] = fun(inst[1], reg[inst[2]]);
    new_reg
}

fn addr(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instrr(inst, reg, &|a, b| a + b)
}

fn addi(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instri(inst, reg, &|a, b| a + b)
}

fn mulr(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instrr(inst, reg, &|a, b| a * b)
}

fn muli(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instri(inst, reg, &|a, b| a * b)
}

fn banr(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instrr(inst, reg, &|a, b| a & b)
}

fn bani(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instri(inst, reg, &|a, b| a & b)
}

fn borr(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instrr(inst, reg, &|a, b| a | b)
}

fn boni(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instri(inst, reg, &|a, b| a | b)
}

fn setr(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instrr(inst, reg, &|a, _| a)
}

fn seti(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instir(inst, reg, &|a, _| a)
}

fn gtir(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instir(inst, reg, &|a, b| (a > b) as usize)
}

fn gtri(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instri(inst, reg, &|a, b| (a > b) as usize)
}

fn gtrr(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instrr(inst, reg, &|a, b| (a > b) as usize)
}

fn eqir(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instir(inst, reg, &|a, b| (a == b) as usize)
}

fn eqri(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instri(inst, reg, &|a, b| (a == b) as usize)
}

fn eqrr(inst: &Vec<usize>, reg: &Vec<usize>) -> Vec<usize> {
    instrr(inst, reg, &|a, b| (a == b) as usize)
}
