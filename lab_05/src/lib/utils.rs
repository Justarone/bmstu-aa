use chrono::{DateTime, Utc};
use log::debug;
use rand::{thread_rng, Rng};
use std::env;
use std::io::prelude::*;
use termion::{color, style};

use super::additional_structs::StrPat;
use super::task::RabinKarpTask;
use super::task::NUMBER_OF_MEASURMENTS;

const DEFAULT_N: usize = 10;
const DEFAULT_STRING_LEN: usize = 200000;
const DEFAULT_PATTERN_LEN: usize = 200;
const RANGE: usize = DEFAULT_PATTERN_LEN * 100;
const FROM: usize = 0;

pub fn generate_string_of_size(size: usize) -> Vec<char> {
    let mut rng = thread_rng();
    (0..size)
        .map(|_| rng.gen_range(FROM, FROM + RANGE))
        .map(|uint| if uint == FROM { 'a' } else { 'b' })
        .collect::<Vec<char>>()
}

pub fn generate_data() -> Vec<RabinKarpTask> {
    let string_len = get_string_len();
    let pattern_len = get_pattern_len();
    let n = get_n();
    println!(
        "Queue length: {}, String length: {}, Pattern length: {}",
        n, string_len, pattern_len
    );
    (0..n)
        .map(|_| {
            RabinKarpTask::new(
                generate_string_of_size(string_len),
                generate_string_of_size(pattern_len),
            )
        })
        .collect()
}

pub fn read_data() -> Vec<RabinKarpTask> {
    let mut n = String::new();
    print!("Введите кол-во элементов, которое будет в очереди: ");
    std::io::stdout().flush().expect("Can't flush stdout");
    std::io::stdin().read_line(&mut n).expect("Read string");
    let n = n.trim().parse::<usize>().unwrap();

    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        let mut string = String::new();
        print!(
            "Введите {} строку, в которой будет происходить поиск подстроки: ",
            i + 1
        );
        std::io::stdout().flush().expect("Can't flush stdout");
        std::io::stdin()
            .read_line(&mut string)
            .expect("Read string");
        string.pop();

        let mut pattern = String::new();
        print!("Введите {} подстроку для поиска в строке: ", i + 1);
        std::io::stdout().flush().expect("Can't flush stdout");
        std::io::stdin()
            .read_line(&mut pattern)
            .expect("Read string");
        pattern.pop();

        result.push(RabinKarpTask::new(
            string.chars().collect(),
            pattern.chars().collect(),
        ));
        println!("");
    }

    result
}

pub fn show_result(result: &[usize], data: &StrPat) {
    let patlen = data.pattern.len();
    let text = &data.string;

    for &pos in result {
        debug!(
            "{}{}{}{}{}{}",
            chars_to_string(&text[0..pos]),
            color::Bg(color::Green),
            style::Bold,
            chars_to_string(&text[pos..(pos + patlen)]),
            style::Reset,
            chars_to_string(&text[(pos + patlen)..])
        );
    }
}

pub fn get_n() -> usize {
    let n = env::var("N");
    match n {
        Ok(n) => n.trim().parse().expect("Parse string to usize"),
        Err(_) => DEFAULT_N,
    }
}

pub fn get_string_len() -> usize {
    let string_len = env::var("STRING_LEN");
    match string_len {
        Ok(string_len) => string_len.trim().parse().expect("Parse string to usize"),
        Err(_) => DEFAULT_STRING_LEN,
    }
}

pub fn get_pattern_len() -> usize {
    let pattern_len = env::var("PATTERN_LEN");
    match pattern_len {
        Ok(pattern_len) => pattern_len.trim().parse().expect("Parse string to usize"),
        Err(_) => DEFAULT_PATTERN_LEN,
    }
}

pub fn chars_to_string(arr: &[char]) -> String {
    arr.iter().collect()
}

enum Part {
    Part1,
    Part2,
    Part3,
}

enum EventType {
    Start,
    End,
}

struct Event {
    task_number: usize,
    part: Part,
    event_type: EventType,
    timestamp: DateTime<Utc>,
}

impl Event {
    fn from_task_time(task_number: usize, time: DateTime<Utc>, pos: usize) -> Self {
        Self {
            timestamp: time.clone(),
            task_number,
            part: match pos / 2 {
                0 => Part::Part1,
                1 => Part::Part2,
                2 => Part::Part3,
                _ => unreachable!("only 3 parts in task"),
            },
            event_type: match pos % 2 {
                0 => EventType::Start,
                1 => EventType::End,
                _ => unreachable!("only 2 types of events"),
            },
        }
    }
}

fn print_task_number(number: usize) {
    print!(
        "{}{}{} Task №{:<3} {}",
        style::Bold,
        color::Bg(color::White),
        color::Fg(color::Black),
        number + 1,
        style::Reset
    );
}

fn print_event_part(part: Part) {
    print!("{}", style::Bold);
    match part {
        Part::Part1 => print!(
            "{}{} Part 1 ",
            color::Bg(color::Yellow),
            color::Fg(color::Black)
        ),
        Part::Part2 => print!("{} Part 2 ", color::Bg(color::Cyan)),
        Part::Part3 => print!("{} Part 3 ", color::Bg(color::Magenta)),
    }
    print!("{}", style::Reset);
}

fn print_event_type(event_type: EventType) {
    print!("{}", style::Bold);
    match event_type {
        EventType::Start => print!("{} Start ", color::Bg(color::Green)),
        EventType::End => print!("{}  End  ", color::Bg(color::Red)),
    }
    print!("{}", style::Reset);
}

fn print_event_timestamp(timestamp: DateTime<Utc>) {
    print!(
        "{}{} {} {}",
        style::Bold,
        color::Bg(color::Blue),
        timestamp,
        style::Reset
    );
}

fn print_sorted_events(events: Vec<Event>) {
    for event in events {
        print_task_number(event.task_number);
        print_event_part(event.part);
        print_event_type(event.event_type);
        print_event_timestamp(event.timestamp);
        println!("");
    }
}

pub fn show_log(times: Vec<[DateTime<Utc>; NUMBER_OF_MEASURMENTS]>) {
    let mut events = Vec::with_capacity(times.len() * 6);
    for (task_number, parts_times) in times.iter().enumerate() {
        for (pos, time) in parts_times.iter().enumerate() {
            events.push(Event::from_task_time(task_number, time.clone(), pos));
        }
    }
    events.sort_by_key(|event| event.timestamp);
    print_sorted_events(events);
}
