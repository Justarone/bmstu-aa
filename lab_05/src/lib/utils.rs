use std::env;
use std::io::prelude::*;
use termion::{ color, style };
use rand::random;

use super::additional_structs::RabinKarpTaskResult;
use super::task::RabinKarpTask;

const DEFAULT_N: usize = 10;
const DEFAULT_STRING_LEN: usize = 200;
const DEFAULT_PATTERN_LEN: usize = 1;

pub fn generate_string_of_size(size: usize) -> Vec<char> {
    (0..size).map(|_| (30_u8 + (random::<f32>() * 52.0) as u8) as char).collect()
}

pub fn generate_data(string_len: usize, pattern_len: usize, n: usize) -> Vec<RabinKarpTask> {
    (0..n).map(|_| RabinKarpTask::new(generate_string_of_size(string_len),
        generate_string_of_size(pattern_len))).collect()
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
        print!("Введите {} строку, в которой будет происходить поиск подстроки: ", i + 1);
        std::io::stdout().flush().expect("Can't flush stdout");
        std::io::stdin().read_line(&mut string).expect("Read string");
        string.pop();

        let mut pattern = String::new();
        print!("Введите {} подстроку для поиска в строке: ", i + 1);
        std::io::stdout().flush().expect("Can't flush stdout");
        std::io::stdin().read_line(&mut pattern).expect("Read string");
        pattern.pop();

        result.push(RabinKarpTask::new(string.chars().collect(), pattern.chars().collect()));
        println!("");
    }

    result
}

pub fn show_result(result: &RabinKarpTaskResult) {
    let data = &result.data;
    let result = &result.result;

    let patlen = data.pattern.len();
    let text = &data.string;

    for &pos in result {
        println!("{}{}{}{}{}{}", chars_to_string(&text[0..pos]), color::Bg(color::Green), style::Bold,
            chars_to_string(&text[pos..(pos + patlen)]), style::Reset, chars_to_string(&text[(pos + patlen)..]));
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
