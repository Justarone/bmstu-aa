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

    let mut conv = Conveyor3::new(data);

    let mut i = 0;
    while let Some(res) = conv.recv() {
        i += 1;
        println!("Тест №{}\nСтрока   : {}\nПодстрока: {}", i, res.data.string, res.data.pattern);
        show_result(&res);
    }

    println!("КОНЕЦ ТЕСТОВ");

    let mut metrics = conv.get_metrics();
    if let Some(metrics) = metrics.take() {
        for (i, layer) in metrics.iter().enumerate() {
            println!("Metrics №{}", i + 1);
            layer.print_all();
        }
    }
}

pub fn run_interactive() {
    let data = utils::read_data();
    let mut conv = Conveyor3::new(data);

    let mut i = 0;
    while let Some(res) = conv.recv() {
        i += 1;
        println!("Тест №{}\nСтрока   : {}\nПодстрока: {}", i, res.data.string, res.data.pattern);
        show_result(&res);
    }

    println!("КОНЕЦ ТЕСТОВ");

    let mut metrics = conv.get_metrics();
    if let Some(metrics) = metrics.take() {
        for (i, layer) in metrics.iter().enumerate() {
            println!("Metrics №{}", i + 1);
            layer.print_all();
        }
    }
}
