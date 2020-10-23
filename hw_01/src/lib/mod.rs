use log::{ info };
use std::time::Instant;

mod utils;
mod algorithm;

pub fn run_tests() {
    let string_len = utils::get_string_len();
    let pattern_len = utils::get_pattern_len();
    let n = utils::get_n();
    let data = utils::generate_data(string_len, pattern_len);
    info!("Будет проведено {} замеров!", n);

    let time = Instant::now();
    for i in 0..n {
        info!("Замер №{}", i);
        algorithm::rabin_karp(&data);
    }
    
    println!("Среднее время прогона: {}", time.elapsed().as_nanos())
}

pub fn run_interactive() {
    let data = utils::read_data();
    let result = algorithm::rabin_karp(&data);
    utils::show_result(&result, &data);
}
