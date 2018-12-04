extern crate chrono;
extern crate regex;
extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use chrono::prelude::*;
use regex::Regex;
use itertools::Itertools;

fn main() {
    let mut f = File::open("data/day_04_input")
        .expect("File not found");

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Something went wrong reading the file");


    let sleepiest = sleepiest_guard(buffer.lines());
    println!("Sleepiest guard: {}", sleepiest.0 * sleepiest.1);

    let sleepiest = most_frequent_sleeper(buffer.lines());
    println!("Most frequent sleeper: {}", sleepiest.0 * sleepiest.1);
}

fn most_frequent_sleeper(lines: std::str::Lines) -> (usize, usize) {
    let mut logs = generate_sleepmap(lines);

    let (sleepiest_id, sleepiest_minute, _) = logs.drain()
        .fold((0, 0, 0), |(best_guard_id, best_minute, best_count), (guard_id, minutes)| {
            let (minute, count) = minutes.iter()
                .enumerate()
                .fold((0, 0), |(pos, val), (i, x)| {
                    if x > &val {
                        (i, *x)
                    } else {
                        (pos, val)
                    }
                });

            if count > best_count {
                (guard_id, minute, count)
            } else {
                (best_guard_id, best_minute, best_count)
            }
        });

    (sleepiest_id, sleepiest_minute)
}

fn sleepiest_guard(lines: std::str::Lines) -> (usize, usize) {
    let logs = generate_sleepmap(lines);

    let (sleepiest_id, _) = logs.iter()
        .fold((0, 0), |(best_guard_id, best_sum), (guard_id, values)| {
            let sum = values.iter().sum();

            if sum > best_sum {
                (*guard_id, sum)
            } else {
                (best_guard_id, best_sum)
            }
        });

    let (sleepiest_minute, _) = logs.get(&sleepiest_id)
        .expect("")
        .iter()
        .enumerate()
        .fold((0, 0), |(pos, val), (i, x)| {
            if x > &val {
                (i, *x)
            } else {
                (pos, val)
            }
        });

    (sleepiest_id, sleepiest_minute)
}

fn generate_sleepmap(lines: std::str::Lines) -> HashMap<usize, [usize; 60]> {
    let logs: SleepMap = lines
        .map(string_to_duty_log)
        .sorted_by(|a, b| a.timestamp.cmp(&b.timestamp))
        .iter()
        .fold(SleepMap::new(), |mut map, log| {
            match log.log_type {
                LogType::ShiftBegins => {
                    let key = log.guard_id.expect("");
                    map.map.entry(key).or_insert([0; 60]);
                    map.last_key = Some(key);
                    map
                },
                LogType::FallsAsleep => {
                    map.last_minute = Some(log.timestamp.minute() as usize);
                    map
                },
                LogType::WakesUp => {
                    let last = map.last_minute.unwrap_or_default();

                    map.map.entry(map.last_key.expect(""))
                        .and_modify(|v| {
                            for minute in last..(log.timestamp.minute() as usize) {
                                v[minute] += 1;
                            }
                        });
                    map
                }
            }
        });

    logs.map
}

#[derive(Debug, PartialEq)]
enum LogType {
    ShiftBegins,
    FallsAsleep,
    WakesUp
}

fn string_to_logtype(log: &str) -> Option<LogType> {
    match log {
        "begins shift" => Some(LogType::ShiftBegins),
        "falls asleep" => Some(LogType::FallsAsleep),
        "wakes up" => Some(LogType::WakesUp),
        _ => None
    }
}

#[derive(Debug, PartialEq)]
struct DutyLog {
    guard_id: Option<usize>,
    log_type: LogType,
    timestamp: NaiveDateTime
}

struct SleepMap {
    last_key: Option<usize>,
    last_minute: Option<usize>,
    map: HashMap<usize, [usize; 60]>
}

impl SleepMap {
    fn new() -> SleepMap {
        SleepMap {
            last_key: None,
            last_minute: None,
            map: HashMap::new()
        }
    }
}

fn string_to_duty_log(row: &str) -> DutyLog {
    let re = Regex::new(r"\[([0-9]+-[0-9]+-[0-9]+ [0-9]+:[0-9]+)\] Guard #([0-9]+) ([a-zA-Z]+.*)|\[([0-9]+-[0-9]+-[0-9]+ [0-9]+:[0-9]+)\] ([a-zA-Z]+.*)")
        .unwrap();

    let values = re.captures(row).unwrap();

    let guard_id = values.get(2).map_or(None, |m| -> Option<usize> { Some(m.as_str().parse().expect("")) });

    let (timestamp, log_type) = if guard_id == None {
        (values.get(4).map(|m| NaiveDateTime::parse_from_str(m.as_str(), "%Y-%m-%d %H:%M").expect("")).expect(""),
         values.get(5).map(|m| string_to_logtype(m.as_str()).expect("")).expect(""))
    } else {
        (values.get(1).map(|m| NaiveDateTime::parse_from_str(m.as_str(), "%Y-%m-%d %H:%M").expect("")).expect(""),
         values.get(3).map(|m| string_to_logtype(m.as_str()).expect("")).expect(""))
    };

    DutyLog {
        guard_id: guard_id,
        log_type: log_type,
        timestamp: timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duty_log() {
        assert_eq!(string_to_duty_log("[1518-11-22 00:00] Guard #1231 begins shift"),
                   DutyLog {
                       guard_id: Some(1231),
                       log_type: string_to_logtype("begins shift").expect(""),
                       timestamp: NaiveDateTime::parse_from_str("1518-11-22 00:00", "%Y-%m-%d %H:%M").expect("")
                   }
        );
    }

    #[test]
    fn test_sleepiest_guard() {
        let test_logs = "[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:05] falls asleep\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:55] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep\n[1518-11-02 00:50] wakes up\n[1518-11-03 00:05] Guard #10 begins shift\n[1518-11-03 00:24] falls asleep\n[1518-11-03 00:29] wakes up\n[1518-11-04 00:02] Guard #99 begins shift\n[1518-11-04 00:36] falls asleep\n[1518-11-04 00:46] wakes up\n[1518-11-05 00:03] Guard #99 begins shift\n[1518-11-05 00:45] falls asleep\n[1518-11-05 00:55] wakes up";

        let sleepiest = sleepiest_guard(String::from(test_logs).lines());
        assert_eq!(sleepiest.0 * sleepiest.1, 240);

        let sleepiest = most_frequent_sleeper(String::from(test_logs).lines());
        assert_eq!(sleepiest.0 * sleepiest.1, 4455);
    }
}
