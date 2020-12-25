use log::{debug, info};
use std::time::Instant;

mod additional_structs;
mod conveyor;
mod task;
mod utils;

use task::RabinKarpTask;

use conveyor::Conveyor3;
use utils::{chars_to_string, show_log, show_result};

pub fn run_tests() {
    let data = utils::generate_data();
    run(data);
}

pub fn run_comparison() {
    let data = utils::generate_data();
    compare(data);
}

pub fn run_interactive() {
    let data = utils::read_data();
    run(data);
}

fn compare(mut data: Vec<RabinKarpTask>) {
    let time = Instant::now();
    for task in data.iter_mut() {
        task.run_all();
    }
    println!("Linear time: {}", time.elapsed().as_nanos());
    let conv_parall = Conveyor3::new(data.clone());
    let time = Instant::now();
    while let Some(_) = conv_parall.recv() {}
    println!("Parallel time: {}", time.elapsed().as_nanos());
}

fn run(data: Vec<RabinKarpTask>) {
    let conv = Conveyor3::new(data);

    let mut i = 0;
    let mut times_arr = Vec::new();
    while let Some(res) = conv.recv() {
        i += 1;
        debug!(
            "Тест №{}\nСтрока   : {}\nПодстрока: {}",
            i,
            chars_to_string(&res.data.string),
            chars_to_string(&res.data.pattern)
        );
        show_result(&res.result, &res.data);
        times_arr.push(res.times);
    }

    show_log(times_arr);

    info!("КОНЕЦ ТЕСТОВ");
}
