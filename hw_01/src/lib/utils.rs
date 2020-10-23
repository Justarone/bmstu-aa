use std::env;
use super::algorithm::StrPat;
use std::io::prelude::*;
use termion::{ color, style };
use rand::random;

const DEFAULT_N: usize = 10;
const DEFAULT_STRING_LEN: usize = 200;
const DEFAULT_PATTERN_LEN: usize = 20;

pub fn generate_string_of_size(size: usize) -> String {
    (0..size).map(|_| (0x61u8 + (random::<f32>() * 22.0) as u8) as char).collect()
}

pub fn generate_data(string_len: usize, pattern_len: usize) -> StrPat {
    StrPat::new(generate_string_of_size(string_len),
        generate_string_of_size(pattern_len))
}

pub fn read_data() -> StrPat {
    let mut string = String::new();
    print!("Введите строку, в которой будет происходить поиск подстроки: ");
    std::io::stdout().flush().expect("Can't flush stdout");
    std::io::stdin().read_line(&mut string).expect("Read string");
    string.pop();

    let mut pattern = String::new();
    print!("Введите подстроку для поиска в строке: ");
    std::io::stdout().flush().expect("Can't flush stdout");
    std::io::stdin().read_line(&mut pattern).expect("Read string");
    pattern.pop();

    StrPat::new(string, pattern)
}

pub fn show_result(result: &[usize], data: &StrPat) {
    let patlen = data.pattern.len();
    let text = &data.string;

    for pos in result {
        println!("{}{}{}{}{}{}", &text[0..*pos], color::Bg(color::Green), style::Bold,
            &text[*pos..(*pos + patlen)], style::Reset, &text[(*pos + patlen)..]);
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
