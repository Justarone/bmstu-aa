use log::{ info, debug };

mod utils;
mod conveyor;
mod task;
mod additional_structs;

use task::RabinKarpTask;

use conveyor::Conveyor3;
use utils::{ chars_to_string, show_result, show_log };

pub fn run_tests() {
    let data = utils::generate_data();
    run(data);
}

pub fn run_interactive() {
    let data = utils::read_data();
    run(data);
}

fn run(data: Vec<RabinKarpTask>) {
    let conv = Conveyor3::new(data);

    let mut i = 0;
    let mut times_arr = Vec::new();
    while let Some(res) = conv.recv() {
        i += 1;
        debug!("Тест №{}\nСтрока   : {}\nПодстрока: {}", i, chars_to_string(&res.data.string),
            chars_to_string(&res.data.pattern));
        show_result(&res.result, &res.data);
        times_arr.push(res.times);
    }

    show_log(times_arr);

    info!("КОНЕЦ ТЕСТОВ");
}
