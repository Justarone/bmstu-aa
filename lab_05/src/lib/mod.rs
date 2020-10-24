use log::{ info };

mod utils;
mod conveyor;
mod task;
mod additional_structs;

use conveyor::Conveyor3;
use utils::show_result;

pub fn run_tests() {
    let string_len = utils::get_string_len();
    let pattern_len = utils::get_pattern_len();
    let n = utils::get_n();
    let data = utils::generate_data(string_len, pattern_len, n);
    info!("Очередь длинной {}, длины строк - {}, длины подстрок - {}", n, string_len, pattern_len);

    let conv = Conveyor3::new(data);

    for res in conv {
        show_result(&res);
    }

    println!("Конец тестов");
}

pub fn run_interactive() {
    let data = utils::read_data();
    let conv = Conveyor3::new(data);

    for res in conv {
        show_result(&res);
    }

    println!("Конец тестов");
}
